mod blockchain;
mod account;
mod txs;

use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tracing::{info};
use anyhow::Result;
use tonic::transport::Server;
use proto::rpc::account_service_server::AccountServiceServer;
use proto::rpc::chain_service_server::ChainServiceServer;
use proto::rpc::transactions_service_server::TransactionsServiceServer;
use traits::{Blockchain, StateDB};
use txpool::TxPool;
use types::config::EnvironmentConfig;
use crate::account::AccountServiceImpl;
use crate::blockchain::ChainServiceImpl;
use crate::txs::TransactionsServiceImpl;

pub struct RPC;

pub async fn start_rpc_server(blockchain: Arc<dyn Blockchain>, state: Arc<dyn StateDB>, txpool: Arc<RwLock<TxPool>>, env: Arc<EnvironmentConfig>) -> Result<()> {
    let host = env.host();
    let port = env.rpc_port();
    let addr = SocketAddr::new(host.parse()?, port);
    let chain_service = ChainServiceImpl::new(blockchain);
    let account_service = AccountServiceImpl::new(state, txpool.clone());
    let transaction_service = TransactionsServiceImpl::new(txpool);

    info!(addr = ?addr, "RPC server running at");
    Server::builder()
        .add_service(ChainServiceServer::new(chain_service))
        .add_service(AccountServiceServer::new(account_service))
        .add_service(TransactionsServiceServer::new(transaction_service))
        .serve(addr)
        .await?;
    Ok(())
}