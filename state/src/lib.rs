use std::collections::{BTreeMap};
use std::option::Option::Some;
use std::path::{Path, PathBuf};
use std::sync::{Arc};

use anyhow::{Result};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tempdir::TempDir;
use tiny_keccak::{Hasher};

use codec::impl_codec;
use codec::{Codec, Decoder, Encoder};
use primitive_types::{H160, H256};
use smt::proof::Proof;
use smt::Tree;
use traits::StateDB;
use transaction::{NoncePricedTransaction, TransactionsByNonceAndPrice};
use types::account::{AccountState};
use types::tx::{Transaction, TransactionKind};
use types::Hash;
use crate::error::StateError;

mod error;

const GENESIS_ROOT: [u8; 32] = [0; 32];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReadProof {
    proof: Proof,
    root: H256,
}

#[derive(Clone)]
pub struct State {
    trie: Arc<Tree<H160, AccountState>>,
    path: PathBuf,
    read_only: bool,
}

unsafe impl Sync for State {}

unsafe impl Send for State {}

impl StateDB for State {
    fn nonce(&self, address: &H160) -> u64 {
        self.trie.get(address).map(|account_state|
            account_state.and_then(|account_state| Some(account_state.nonce))
        ).unwrap_or_default().unwrap_or_default()
    }

    fn account_state(&self, address: &H160) -> AccountState {
        self.trie.get(address).unwrap_or_default().unwrap_or_default()
    }

    fn balance(&self, address: &H160) -> u128 {
        self.account_state(address).free_balance
    }

    fn credit_balance(&self, address: &H160, amount: u128) -> Result<Hash> {
        let action = StateOperation::CreditBalance {
            account: *address,
            amount,
            tx_hash: [0; 32],
        };
        self.apply_operation(action)?;
        Ok(self.root_hash().unwrap())
    }

    fn debit_balance(&self, address: &H160, amount: u128) -> Result<Hash> {
        let action = StateOperation::DebitBalance {
            account: *address,
            amount,
            tx_hash: [0; 32],
        };
        self.apply_operation(action)?;
        Ok(self.root_hash().unwrap())
    }

    fn reset(&self, root: H256) -> Result<()> {
        self.trie.reset(root)
    }


    fn apply_txs(&self, txs: Vec<Transaction>) -> Result<Hash> {
        self.apply_txs(txs)?;
        self.root_hash()
    }

    fn root(&self) -> Hash {
        self.root_hash().unwrap()
    }

    fn commit(&self) -> Result<()> {
        self.trie.commit().map(|_| ())
    }

    fn snapshot(&self) -> Result<Arc<dyn StateDB>> {
        Ok(self.get_sate_at(H256::from(self.root()))?)
    }
}

impl State {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let trie = Tree::open(path.as_ref())?;
        Ok(Self {
            trie: Arc::new(trie),
            path: PathBuf::from(path.as_ref()),
            read_only: false,
        })
    }

    pub fn apply_txs(&self, txs: Vec<Transaction>) -> Result<()> {
        let mut accounts: BTreeMap<H160, TransactionsByNonceAndPrice> = BTreeMap::new();
        let mut states: BTreeMap<H160, AccountState> = BTreeMap::new();

        for tx in txs {
            let mut txs = accounts.entry(tx.from()).or_default();
            txs.insert(NoncePricedTransaction(tx));
        }

        for (acc, _) in accounts.iter() {
            let current_state = self.trie.get(&acc)?.unwrap_or_default();
            states.insert(*acc, current_state);
        }

        for (_, txs) in accounts {
            for tx in txs {
                self.apply_transaction(tx.0, &mut states)?;
            }
        }

        for (acc, state) in states {
            self.trie.put(acc, state)?;
        }
        Ok(())
    }

    fn apply_transaction(
        &self,
        transaction: Transaction,
        states: &mut BTreeMap<H160, AccountState>,
    ) -> Result<()> {
        //TODO: verify transaction (probably)
        for action in get_operations(&transaction) {
            let address = action.get_address();
            let account_state = states.get(&address).map(|state| state.clone()).unwrap_or_default();
            let new_account_state = self.apply_action(&action, account_state)?;
            states.insert(address, new_account_state);
        }
        Ok(())
    }

    fn apply_operation(&self, action: StateOperation) -> Result<()> {
        let current_account_state = self.get_account_state(&action.get_address())?;
        let new_account_state = self.apply_action(&action, current_account_state)?;
        self.trie.put(action.get_address(), new_account_state)?;
        Ok(())
    }

    fn commit(&self) -> Result<()> {
        if !self.read_only {
            self.trie.commit()?;
        }
        Ok(())
    }

    pub fn check_transaction(&self, transaction: &Transaction) -> Result<()> {
        Ok(())
    }

    fn apply_action(
        &self,
        action: &StateOperation,
        account_state: AccountState,
    ) -> Result<AccountState> {
        let mut account_state = account_state;
        match action {
            StateOperation::DebitBalance { amount, .. } => {
                if account_state.free_balance < *amount {
                    return Err(StateError::InsufficientFunds.into());
                }
                account_state.free_balance = account_state.free_balance.saturating_sub(*amount);
                Ok(account_state)
            }
            StateOperation::CreditBalance { amount, .. } => {
                account_state.free_balance = account_state.free_balance.saturating_add(*amount);
                Ok(account_state)
            }
            StateOperation::UpdateNonce { nonce, .. } => {
                if *nonce <= account_state.nonce {
                    return Err(StateError::NonceIsLessThanCurrent.into());
                }
                account_state.nonce = *nonce;
                Ok(account_state)
            }
        }
    }

    fn get_account_state(&self, address: &H160) -> Result<AccountState> {
        Ok(self.trie.get(address).unwrap_or_default().unwrap_or_default())
    }

    pub fn get_sate_at(&self, root: H256) -> Result<Arc<Self>> {
        Ok(Arc::new(State {
            trie: Arc::new(Tree::open_read_only_at_root(self.path.as_path(), &root)?),
            path: self.path.clone(),
            read_only: true,
        }))
    }

    fn get_account_state_with_proof(
        &self,
        address: &H160,
    ) -> Result<(AccountState, ReadProof)> {
        let (account_state, proof) = self.trie.get_with_proof(&address)?;
        let root = self.trie.root()?;
        Ok((account_state, ReadProof { proof, root }))
    }

    pub fn checkpoint<P: AsRef<Path>>(&self, path: P) -> Result<Self> {
        unimplemented!()
    }

    pub fn root_hash(&self) -> Result<Hash> {
        self.trie.root().map(|root| root.to_fixed_bytes())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StateOperation {
    DebitBalance {
        account: H160,
        amount: u128,
        tx_hash: Hash,
    },
    CreditBalance {
        account: H160,
        amount: u128,
        tx_hash: Hash,
    },
    UpdateNonce {
        account: H160,
        nonce: u64,
        tx_hash: Hash,
    },
}

impl StateOperation {
    fn get_address(&self) -> H160 {
        match self {
            StateOperation::DebitBalance { account, .. } => *account,
            StateOperation::CreditBalance { account, .. } => *account,
            StateOperation::UpdateNonce { account, .. } => *account,
        }
    }
}

pub fn get_operations(tx: &Transaction) -> Vec<StateOperation> {
    let mut ops = Vec::new();
    let tx_hash = tx.hash();
    match tx.kind() {
        TransactionKind::Transfer {
            from,
            to,
            amount,
            fee,
            ..
        } => {
            ops.push(StateOperation::DebitBalance {
                account: H160::from(from),
                amount: *amount + *fee,
                tx_hash,
            });
            ops.push(StateOperation::CreditBalance {
                account: H160::from(to),
                amount: *amount,
                tx_hash,
            });
            ops.push(StateOperation::UpdateNonce {
                account: H160::from(from),
                nonce: tx.nonce(),
                tx_hash,
            });
        }
    }
    ops
}
impl_codec!(StateOperation);

pub trait MorphCheckPoint {
    fn checkpoint(&self) -> State;
}

#[cfg(test)]
mod tests {
    use tempdir::TempDir;

    use account::create_account;
    use transaction::make_sign_transaction;

    use super::*;

    #[test]
    fn test_morph() {
        let path = TempDir::new("state").unwrap();
        let mut state = State::new(path.path()).unwrap();
        let alice = create_account();
        let bob = create_account();
        let jake = create_account();
        state.credit_balance(&alice.address, 1_000_000).unwrap();
        let mut txs = Vec::new();
        for i in 0..100 {
            let amount = 100;
            let tx = make_sign_transaction(
                &alice,
                i + 1,
                TransactionKind::Transfer {
                    from: alice.address.to_fixed_bytes(),
                    to: bob.address.to_fixed_bytes(),
                    amount,
                    fee: (amount as f64 * 0.01) as u128,
                },
            )
                .unwrap();
            txs.push(tx);
        }
        state.apply_txs(txs).unwrap();

        println!("Alice: {:#?}", state.account_state(&alice.address));
        println!("Bob: {:#?}", state.account_state(&bob.address));

        let read_state = state.snapshot().unwrap();
        println!("Read Alice: {:#?}", read_state.account_state(&alice.address));
        println!("Read Bob: {:#?}", read_state.account_state(&bob.address));
    }
}
