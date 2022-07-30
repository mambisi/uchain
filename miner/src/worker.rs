use std::ops::Deref;
use std::sync::atomic::{AtomicBool, AtomicI8, Ordering};
use std::sync::{Arc, RwLock};

use anyhow::Result;
use blockchain::block_storage::BlockStorage;
use chrono::Utc;
use tokio::sync::mpsc::UnboundedSender;
use blockchain::chain_state::ChainState;

use merkle::Merkle;
use p2p::peer_manager::NetworkState;
use primitive_types::{H160, H256, U256};
use tracing::{debug, info, warn};
use traits::{Blockchain, ChainHeadReader, Consensus, StateDB};
use txpool::tx_lookup::AccountSet;
use txpool::{ResetRequest, TxPool};
use types::block::{Block, BlockHeader};
use types::events::LocalEventMessage;
use types::tx::Transaction;
use types::Address;

pub const SHUTDOWN: i8 = -1;
pub const RESET: i8 = 0;
pub const PAUSE: i8 = 1;
pub const START: i8 = 2;

pub fn start_worker(
    coinbase: H160,
    lmpsc: UnboundedSender<LocalEventMessage>,
    consensus: Arc<dyn Consensus>,
    txpool: Arc<RwLock<TxPool>>,
    chain: Arc<ChainState>,
    network: Arc<NetworkState>,
    chain_header_reader: Arc<dyn ChainHeadReader>,
    interrupt: Arc<AtomicI8>,
) -> Result<()> {
    let is_running: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let mut current_block_template: Option<(BlockHeader, Vec<Transaction>)> = None;
    info!(miner = ?coinbase, "mine worker started running");
    loop {
        let _ = is_running.load(Ordering::Acquire);
        let i = interrupt.load(Ordering::Acquire);
        if i == SHUTDOWN {
            is_running.store(false, Ordering::Release);
            warn!(reason = i, "⛔ mine worker shutting down");
            return Ok(());
        }

        let network_head = network
            .network_head()
            .map(|block| block.level)
            .unwrap_or_default();
        let node_head = chain
            .current_header()
            .map(|block| block.map(|block| block.raw.level).unwrap_or_default())
            .unwrap_or_default();

        if network_head > node_head {
            continue;
        }

        let (mut block_template, txs) = {
            let (head, txs) = make_block_template(
                coinbase.to_fixed_bytes(),
                consensus.clone(),
                txpool.clone(),
                chain.get_current_state()?,
                chain.clone(),
                chain_header_reader.clone(),
            )?;
            current_block_template = Some((head.clone(), txs.clone()));
            debug!(coinbase = ?coinbase, txs_count = txs.len(), "🚧 mining a new block");
            (head, txs)
        };

        loop {
            if i == SHUTDOWN {
                break
            }

            let network_head = network
                .network_head();

            let node_head = chain
                .current_header()?.unwrap();

            let network_height = network_head.map(|header| header.level).unwrap_or_default();
            let node_height = node_head.raw.level;

            if network_height > node_height {
                break;
            }

            if U256::from(block_template.nonce) + U256::one() > U256::from(u128::MAX) {
                let nonce = U256::from(block_template.nonce) + U256::one();
                let mut mix_nonce = U256::from(block_template.mix_nonce);
                mix_nonce += nonce;
                let mut out = [0; 32];
                mix_nonce.to_big_endian(&mut out);
                block_template.mix_nonce = out;
                block_template.nonce = 0
            }
            block_template.nonce += 1;
            block_template.time = Utc::now().timestamp() as u32;

            if consensus
                .verify_header(chain_header_reader.clone(), &block_template)
                .is_ok()
            {
                let hash = block_template.hash();
                let level = block_template.level;

                let node_head = chain
                    .current_header()
                    .map(|block| block.map(|block| block.raw.level).unwrap_or_default())
                    .unwrap_or_default();

                if node_head >= level {
                    break;
                }

                info!(level = level, hash = ?hex::encode(hash), parent_hash = ?format!("{}", H256::from(block_template.parent_hash)), "⛏ mined new block");
                let block = Block::new(block_template, txs);
                interrupt.store(RESET, Ordering::Release);
                let blocks = vec![block.clone()];
                chain.put_chain(consensus.clone(), Box::new(blocks.into_iter()), txpool.clone())?;
                lmpsc.send(LocalEventMessage::MindedBlock(block))?;
                break;
            }
        }
    }

    warn!("miner shutdown");
}

fn pack_pending_txs(txpool: Arc<RwLock<TxPool>>) -> Result<([u8; 32], Vec<Transaction>)> {
    let txpool = txpool.read().map_err(|e| anyhow::anyhow!("{}", e))?;
    let mut tsx = Vec::new();
    let mut merkle = Merkle::default();
    for (_, list) in txpool.pending() {
        for tx in list.iter() {
            merkle.update(&tx.hash())?;
        }
        tsx.extend(list.iter().map(|tx_ref| tx_ref.deref().clone()));
    }

    let merkle_root = match merkle.finalize() {
        None => [0; 32],
        Some(root) => *root,
    };

    return Ok((merkle_root, tsx))
}

fn make_block_template(
    coinbase: Address,
    consensus: Arc<dyn Consensus>,
    txpool: Arc<RwLock<TxPool>>,
    state: Arc<dyn StateDB>,
    chain: Arc<dyn Blockchain>,
    chain_header_reader: Arc<dyn ChainHeadReader>,
) -> Result<(BlockHeader, Vec<Transaction>)> {
    let parent_header = match chain.current_header()? {
        None => consensus.get_genesis_header(),
        Some(header) => header.raw,
    };
    let mut state = state.state_at(H256::from(parent_header.state_root))?;
    let (merkle_root, txs) = pack_pending_txs(txpool.clone())?;
    let mut mix_nonce = [0; 32];
    U256::one().to_big_endian(&mut mix_nonce);
    let time = Utc::now().timestamp() as u32;
    let mut header = BlockHeader {
        parent_hash: parent_header.hash(),
        merkle_root,
        state_root: [0; 32],
        mix_nonce,
        coinbase,
        difficulty: 0,
        chain_id: 0,
        level: parent_header.level + 1,
        time,
        nonce: 0,
    };
    consensus.prepare_header(chain_header_reader.clone(), &mut header)?;
    consensus.finalize(chain_header_reader, &mut header, state.clone(), txs.clone())?;
    Ok((header, txs))
}
