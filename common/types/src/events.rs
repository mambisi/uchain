use crate::block::{Block, BlockHeader};
use crate::tx::Transaction;

#[derive(Clone, Debug)]
pub enum LocalEventMessage {
    MindedBlock(Block),
    BroadcastTx(Transaction),
    TxPoolPack(Vec<Transaction>),
    StateChanged {
        current_head: BlockHeader,
    },
    NetworkHighestHeadChanged {
        peer_id: String,
        current_head: BlockHeader,
    },
    NetworkNewPeerConnection {
        stats: (usize, usize)
    },
}
