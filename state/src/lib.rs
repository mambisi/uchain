use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use bincode::{Decode, Encode};
use codec::{Decodable, Encodable};
use serde::{Deserialize, Serialize};

use primitive_types::{Address, H256};
use smt::proof::Proof;
use smt::SparseMerkleTree;
use traits::{StateDB, WasmVMInstance};
use transaction::{NoncePricedTransaction, TransactionsByNonceAndPrice};
use types::account::AccountState;
use types::prelude::{get_address_from_app_id, get_address_from_seed, AppState, TransactionData};
use types::tx::{PaymentTx, SignedTransaction, Transaction};
use types::Hash;

use crate::error::StateError;
use crate::kvdb::KvDB;
use crate::tree::{Op, TrieDB};

mod context;
mod error;
mod kvdb;
mod persistent;
mod store;
mod tree;

const ACCOUNT_DB_NAME: &str = "accs";
const APPDATA_DB_NAME: &str = "data";
const BINDATA_DB_NAME: &str = "bins";

#[derive(Encode, Decode, Clone, Debug)]
pub struct ReadProof {
    proof: Proof,
    root: H256,
}

#[derive(Clone)]
pub struct State {
    trie: Arc<TrieDB<Address, AccountState>>,
    appdata: Arc<KvDB<AppStateKey, SparseMerkleTree>>,
    appsource: Arc<KvDB<H256, Vec<u8>>>,
    path: PathBuf,
    read_only: bool,
}

#[derive(Encode, Decode, Clone, Debug)]
struct AppStateKey(Address, H256);

impl Encodable for AppStateKey {
    fn encode(&self) -> Result<Vec<u8>> {
        bincode::encode_to_vec(self, codec::config()).map_err(|e| anyhow!(e))
    }
}

impl Decodable for AppStateKey {
    fn decode(buf: &[u8]) -> Result<Self> {
        bincode::decode_from_slice(buf, codec::config())
            .map(|(out, _)| out)
            .map_err(|e| anyhow!(e))
    }
}

unsafe impl Sync for State {}

unsafe impl Send for State {}

impl StateDB for State {
    fn nonce(&self, address: &Address) -> u64 {
        self.account_state(address).nonce
    }

    fn account_state(&self, address: &Address) -> AccountState {
        match self.trie.get(address) {
            Ok(Some(account_state)) => account_state,
            _ => AccountState::new(),
        }
    }

    fn balance(&self, address: &Address) -> u64 {
        self.account_state(address).free_balance
    }

    fn credit_balance(&self, address: &Address, amount: u64) -> Result<H256> {
        let mut account_state = self.get_account_state(address)?;
        account_state.free_balance += amount;
        self.trie.put(*address, account_state)?;
        Ok(self.root_hash()?.into())
    }

    fn debit_balance(&self, address: &Address, amount: u64) -> Result<H256> {
        let mut account_state = self.get_account_state(address)?;
        account_state.free_balance -= amount;
        self.trie.put(*address, account_state)?;
        Ok(self.root_hash()?.into())
    }

    fn reset(&self, root: H256) -> Result<()> {
        self.trie.reset(root)
    }

    fn apply_txs(&self, vm: Arc<dyn WasmVMInstance>, txs: &[SignedTransaction]) -> Result<H256> {
        self.apply_txs(vm, txs)?;
        self.root_hash().map(H256::from)
    }

    fn root(&self) -> Hash {
        self.root_hash().unwrap()
    }

    fn commit(&self) -> Result<()> {
        self.commit()
    }

    fn snapshot(&self) -> Result<Arc<dyn StateDB>> {
        Ok(self.get_sate_at(H256::from(self.root()))?)
    }

    fn state_at(&self, root: H256) -> Result<Arc<dyn StateDB>> {
        Ok(self.get_sate_at(root)?)
    }

    fn get_app_data(&self, app_id: Address) -> Result<SparseMerkleTree> {
        let Ok(Some(app_account_state)) = self.trie.get(&app_id) else {
            bail!("app not found")
        };

        let Some(app_root) = app_account_state.app_state.map(|root| H256::from_slice(&root.root_hash)) else {
            bail!("app not initialized")
        };

        Ok(self
            .appdata
            .get(&AppStateKey(app_id, app_root))
            .unwrap_or_else(|_| SparseMerkleTree::new()))
    }

    fn get_app_source(&self, app_id: Address) -> Result<Vec<u8>> {
        let account = self.trie.get(&app_id)?.ok_or(anyhow::anyhow!("app not found"))?;
        let Some(app_state) = account.app_state else {
            bail!("address is not an application address")
        };
        self.appsource.get(&H256::from_slice(&app_state.code_hash))
    }
}

impl State {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let trie = TrieDB::open(path.join(ACCOUNT_DB_NAME).as_path())?;
        let appdata = KvDB::open(path.join(APPDATA_DB_NAME).as_path())?;
        let appsource = KvDB::open(path.join(BINDATA_DB_NAME).as_path())?;
        Ok(Self {
            trie: Arc::new(trie),
            appdata: Arc::new(appdata),
            appsource: Arc::new(appsource),
            path: path.clone(),
            read_only: false,
        })
    }

    pub fn apply_txs(&self, vm: Arc<dyn WasmVMInstance>, txs: &[SignedTransaction]) -> Result<()> {
        let mut accounts: BTreeMap<Address, TransactionsByNonceAndPrice> = BTreeMap::new();
        let mut states: BTreeMap<Address, AccountState> = BTreeMap::new();

        for tx in txs {
            if let std::collections::btree_map::Entry::Vacant(e) = states.entry(tx.from()) {
                let current_state = self.get_account_state(&tx.from())?;
                e.insert(current_state);
            }
            if let std::collections::btree_map::Entry::Vacant(e) = states.entry(tx.to()) {
                let current_state = self.get_account_state(&tx.to())?;
                e.insert(current_state);
            }
            let txs = accounts.entry(tx.from()).or_default();
            txs.insert(NoncePricedTransaction(tx));
        }

        for (_, txs) in accounts {
            for tx in txs.into_iter().map(|tx| tx.0) {
                self.apply_transaction(vm.as_ref(), &mut states, tx)?;
            }
        }
        //TODO; Check accounts for negative balances
        for (acc, state) in states {
            self.trie.put(acc, state)?;
        }
        Ok(())
    }

    fn apply_transaction(
        &self,
        vm: &dyn WasmVMInstance,
        mut states: &mut BTreeMap<Address, AccountState>,
        tx: &SignedTransaction,
    ) -> Result<()> {
        match tx.data() {
            TransactionData::Payment(_) => {
                self.execute_payment_tx(tx, &mut states)?;
            }
            TransactionData::Call(arg) => {
                let app_address = tx.to();
                let state_db = Arc::new(self.clone());
                let changelist = vm.execute_app_tx(state_db, tx.sender(), tx.price(), arg)?;
                for (addr, state) in changelist.account_changes {
                    states.insert(addr, state);
                }
                // Update AppState on Account
                let app_state = states
                    .get_mut(&app_address)
                    .map(|account_state| account_state.app_state.as_mut())
                    .flatten()
                    .ok_or(anyhow::anyhow!("app state not found"))?;
                app_state.root_hash = changelist.storage.root().to_fixed_bytes().to_vec();
                self.appdata.put(
                    AppStateKey(app_address, changelist.storage.root()),
                    changelist.storage,
                )?;
            }
            TransactionData::Create(arg) => {
                let state_db = Arc::new(self.clone());
                let app_address = tx.to();
                let t = self.trie.get(&app_address).ok().flatten();
                if t.is_some() {
                    bail!("app address already exists")
                }
                let code_hash = crypto::keccak256(&arg.binary);
                let changelist = vm.execute_app_create(state_db, tx.sender(), tx.price(), arg)?;
                for (addr, state) in changelist.account_changes {
                    states.insert(addr, state);
                }
                let app_state = states.get_mut(&app_address).ok_or(anyhow::anyhow!("app state not found"))?;
                app_state.app_state = Some(AppState {
                    root_hash: changelist.storage.root().to_fixed_bytes().to_vec(),
                    code_hash: code_hash.as_bytes().to_vec(),
                    creator: tx.from().to_vec(),
                    version: 1,
                });
                self.appsource.put(code_hash, arg.binary.clone())?;
                self.appdata.put(
                    AppStateKey(app_address, changelist.storage.root()),
                    changelist.storage,
                )?;
            }
            TransactionData::Update(_) => {
                unimplemented!("update app transaction not implemented")
            }
            TransactionData::RawData(raw) => {
                println!("[NOT AVAILABLE] Raw DATA: {:?}", hex::encode_raw(raw))
            }
        }

        // Update transaction origin nonce
        let mut from_account_state = states
            .get_mut(&tx.from())
            .ok_or(StateError::AccountNotFound)?;

        let next_nonce = if tx.nonce() > from_account_state.nonce {
            tx.nonce() + 1
        } else {
            from_account_state.nonce + 1
        };
        from_account_state.nonce = next_nonce;
        Ok(())
    }

    pub fn apply_txs_no_commit(
        &self,
        at_root: H256,
        reward: u64,
        coinbase: Address,
        txs: &[SignedTransaction],
    ) -> Result<Hash> {
        let mut accounts: BTreeMap<Address, TransactionsByNonceAndPrice> = BTreeMap::new();
        let mut states: BTreeMap<Address, AccountState> = BTreeMap::new();

        for tx in txs {
            if let std::collections::btree_map::Entry::Vacant(e) = states.entry(tx.from()) {
                let current_state = self.get_account_state_at_root(&at_root, &tx.from())?;
                e.insert(current_state);
            }
            if let std::collections::btree_map::Entry::Vacant(e) = states.entry(tx.to()) {
                let current_state = self.get_account_state_at_root(&at_root, &tx.to())?;
                e.insert(current_state);
            }
            let txs = accounts.entry(tx.from()).or_default();
            txs.insert(NoncePricedTransaction(tx));
        }

        for (_, txs) in accounts {
            for tx in txs {
                self.execute_payment_tx(tx.0, &mut states)?;
            }
        }

        let mut batch: Vec<_> = states.into_iter().map(|(k, v)| Op::Put(k, v)).collect();

        let mut coinbase_account_state = self
            .trie
            .get_at_root(&at_root, &coinbase)
            .unwrap_or_default()
            .unwrap_or_default();
        coinbase_account_state.free_balance += reward;
        batch.push(Op::Put(coinbase, coinbase_account_state));

        self.trie
            .apply_non_commit(&at_root, batch)
            .map(|hash| hash.to_fixed_bytes())
    }

    fn execute_payment_tx(
        &self,
        transaction: &SignedTransaction,
        states: &mut BTreeMap<Address, AccountState>,
    ) -> Result<()> {
        let mut from_account_state = states
            .get_mut(&transaction.from())
            .ok_or(StateError::AccountNotFound)?;
        let amount = transaction.price() + transaction.fees();
        if from_account_state.free_balance < amount {
            return Err(StateError::InsufficientFunds.into());
        }
        from_account_state.free_balance -= amount;
        let mut to_account_state = states
            .get_mut(&transaction.to())
            .ok_or(StateError::AccountNotFound)?;
        to_account_state.free_balance += transaction.price();
        Ok(())
    }

    fn commit(&self) -> Result<()> {
        self.trie.commit(!self.read_only)?;
        Ok(())
    }

    pub fn check_transaction(&self, _transaction: &SignedTransaction) -> Result<()> {
        Ok(())
    }

    fn get_account_state(&self, address: &Address) -> Result<AccountState> {
        match self.trie.get(address) {
            Ok(Some(account_state)) => Ok(account_state),
            _ => Ok(AccountState::new()),
        }
    }

    fn get_account_state_at_root(&self, at_root: &H256, address: &Address) -> Result<AccountState> {
        match self.trie.get_at_root(at_root, address) {
            Ok(Some(account_state)) => Ok(account_state),
            _ => Ok(AccountState::new()),
        }
    }

    pub fn get_sate_at(&self, root: H256) -> Result<Arc<Self>> {
        let trie =
            TrieDB::open_read_only_at_root(self.path.join(ACCOUNT_DB_NAME).as_path(), &root)?;
        let appdata = KvDB::open_read_only_at_root(self.path.join(APPDATA_DB_NAME).as_path())?;
        let appsource = KvDB::open_read_only_at_root(self.path.join(BINDATA_DB_NAME).as_path())?;
        Ok(Arc::new(State {
            trie: Arc::new(trie),
            appdata: Arc::new(appdata),
            appsource: Arc::new(appsource),
            path: self.path.clone(),
            read_only: true,
        }))
    }

    fn get_account_state_with_proof(&self, address: &Address) -> Result<(AccountState, ReadProof)> {
        let (account_state, proof) = self.trie.get_with_proof(address)?;
        let root = self.trie.root()?;
        Ok((account_state, ReadProof { proof, root }))
    }

    pub fn checkpoint<P: AsRef<Path>>(&self, _path: P) -> Result<Self> {
        unimplemented!()
    }

    pub fn root_hash(&self) -> Result<Hash> {
        self.trie.root().map(|root| root.to_fixed_bytes())
    }
}

pub trait MorphCheckPoint {
    fn checkpoint(&self) -> State;
}
