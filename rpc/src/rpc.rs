#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsignedTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<::types::prelude::Transaction>,
    #[prost(bytes = "vec", tag = "2")]
    pub secret_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub hash: ::core::option::Option<::primitive_types::H256>,
    #[prost(message, optional, tag = "2")]
    pub tx: ::core::option::Option<::types::prelude::SignedTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionStatusResponse {
    #[prost(
        enumeration = "::types::prelude::TransactionStatus",
        repeated,
        tag = "1"
    )]
    pub status: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PendingTransactionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pending: ::prost::alloc::vec::Vec<AddressTransactionList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressTransactionList {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<::primitive_types::Address>,
    #[prost(message, optional, tag = "2")]
    pub txs: ::core::option::Option<::types::prelude::TransactionList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxpoolContentResponse {
    #[prost(message, repeated, tag = "1")]
    pub pending: ::prost::alloc::vec::Vec<AddressTransactionList>,
    #[prost(message, repeated, tag = "2")]
    pub queued: ::prost::alloc::vec::Vec<AddressTransactionList>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionHash {
    #[prost(message, optional, tag = "1")]
    pub hash: ::core::option::Option<::primitive_types::H256>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionHashes {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Generated server implementations.
pub mod transactions_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TransactionsServiceServer.
    #[async_trait]
    pub trait TransactionsService: Send + Sync + 'static {
        async fn sign_transaction(
            &self,
            request: tonic::Request<super::UnsignedTransactionRequest>,
        ) -> Result<tonic::Response<super::SignedTransactionResponse>, tonic::Status>;
        async fn sign_send_transaction(
            &self,
            request: tonic::Request<super::UnsignedTransactionRequest>,
        ) -> Result<tonic::Response<super::SignedTransactionResponse>, tonic::Status>;
        async fn send_transaction(
            &self,
            request: tonic::Request<::types::prelude::SignedTransaction>,
        ) -> Result<tonic::Response<super::TransactionHash>, tonic::Status>;
        async fn get_transaction_status(
            &self,
            request: tonic::Request<super::TransactionHashes>,
        ) -> Result<tonic::Response<super::GetTransactionStatusResponse>, tonic::Status>;
        async fn get_pending_transactions(
            &self,
            request: tonic::Request<::types::prelude::Empty>,
        ) -> Result<tonic::Response<super::PendingTransactionsResponse>, tonic::Status>;
        async fn get_txpool_content(
            &self,
            request: tonic::Request<::types::prelude::Empty>,
        ) -> Result<tonic::Response<super::TxpoolContentResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TransactionsServiceServer<T: TransactionsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TransactionsService> TransactionsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TransactionsServiceServer<T>
    where
        T: TransactionsService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rpc.TransactionsService/SignTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SignTransactionSvc<T: TransactionsService>(pub Arc<T>);
                    impl<T: TransactionsService>
                        tonic::server::UnaryService<super::UnsignedTransactionRequest>
                        for SignTransactionSvc<T>
                    {
                        type Response = super::SignedTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnsignedTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sign_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TransactionsService/SignSendTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SignSendTransactionSvc<T: TransactionsService>(pub Arc<T>);
                    impl<T: TransactionsService>
                        tonic::server::UnaryService<super::UnsignedTransactionRequest>
                        for SignSendTransactionSvc<T>
                    {
                        type Response = super::SignedTransactionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnsignedTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sign_send_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SignSendTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TransactionsService/SendTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SendTransactionSvc<T: TransactionsService>(pub Arc<T>);
                    impl<T: TransactionsService>
                        tonic::server::UnaryService<::types::prelude::SignedTransaction>
                        for SendTransactionSvc<T>
                    {
                        type Response = super::TransactionHash;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::types::prelude::SignedTransaction>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_transaction(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendTransactionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TransactionsService/GetTransactionStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionStatusSvc<T: TransactionsService>(pub Arc<T>);
                    impl<T: TransactionsService>
                        tonic::server::UnaryService<super::TransactionHashes>
                        for GetTransactionStatusSvc<T>
                    {
                        type Response = super::GetTransactionStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransactionHashes>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_transaction_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TransactionsService/GetPendingTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct GetPendingTransactionsSvc<T: TransactionsService>(pub Arc<T>);
                    impl<T: TransactionsService>
                        tonic::server::UnaryService<::types::prelude::Empty>
                        for GetPendingTransactionsSvc<T>
                    {
                        type Response = super::PendingTransactionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::types::prelude::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_pending_transactions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPendingTransactionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TransactionsService/GetTxpoolContent" => {
                    #[allow(non_camel_case_types)]
                    struct GetTxpoolContentSvc<T: TransactionsService>(pub Arc<T>);
                    impl<T: TransactionsService>
                        tonic::server::UnaryService<::types::prelude::Empty>
                        for GetTxpoolContentSvc<T>
                    {
                        type Response = super::TxpoolContentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::types::prelude::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_txpool_content(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTxpoolContentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: TransactionsService> Clone for TransactionsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TransactionsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TransactionsService> tonic::server::NamedService for TransactionsServiceServer<T> {
        const NAME: &'static str = "rpc.TransactionsService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountRequest {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<::primitive_types::Address>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountBalanceResponse {
    #[prost(uint64, tag = "1")]
    pub balance: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountNonceResponse {
    #[prost(uint64, tag = "1")]
    pub nonce: u64,
}
/// Generated server implementations.
pub mod account_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AccountServiceServer.
    #[async_trait]
    pub trait AccountService: Send + Sync + 'static {
        async fn get_balance(
            &self,
            request: tonic::Request<super::GetAccountRequest>,
        ) -> Result<tonic::Response<super::GetAccountBalanceResponse>, tonic::Status>;
        async fn get_nonce(
            &self,
            request: tonic::Request<super::GetAccountRequest>,
        ) -> Result<tonic::Response<super::GetAccountNonceResponse>, tonic::Status>;
        async fn get_account_state(
            &self,
            request: tonic::Request<super::GetAccountRequest>,
        ) -> Result<tonic::Response<::types::prelude::AccountState>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AccountServiceServer<T: AccountService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AccountService> AccountServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccountServiceServer<T>
    where
        T: AccountService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rpc.AccountService/GetBalance" => {
                    #[allow(non_camel_case_types)]
                    struct GetBalanceSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::GetAccountRequest> for GetBalanceSvc<T> {
                        type Response = super::GetAccountBalanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_balance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBalanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.AccountService/GetNonce" => {
                    #[allow(non_camel_case_types)]
                    struct GetNonceSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::GetAccountRequest> for GetNonceSvc<T> {
                        type Response = super::GetAccountNonceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_nonce(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNonceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.AccountService/GetAccountState" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountStateSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::GetAccountRequest>
                        for GetAccountStateSvc<T>
                    {
                        type Response = ::types::prelude::AccountState;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_account_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccountStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: AccountService> Clone for AccountServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AccountService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AccountService> tonic::server::NamedService for AccountServiceServer<T> {
        const NAME: &'static str = "rpc.AccountService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentHeadResponse {
    #[prost(message, optional, tag = "1")]
    pub hash: ::core::option::Option<::primitive_types::H256>,
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<::types::prelude::BlockHeader>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHashRequest {
    #[prost(message, optional, tag = "1")]
    pub hash: ::core::option::Option<::primitive_types::H256>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByLevelRequest {
    #[prost(uint32, tag = "1")]
    pub level: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksRequest {
    #[prost(uint32, tag = "1")]
    pub from: u32,
    #[prost(uint32, tag = "2")]
    pub count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlocksResponse {
    #[prost(message, repeated, tag = "1")]
    pub blocks: ::prost::alloc::vec::Vec<::types::prelude::BlockHeader>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockNumberResponse {
    #[prost(uint32, tag = "1")]
    pub level: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainInfo {
    #[prost(string, tag = "1")]
    pub chain: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub genesis_hash: ::core::option::Option<::primitive_types::H256>,
    #[prost(uint32, tag = "3")]
    pub difficulty: u32,
    #[prost(uint32, tag = "4")]
    pub network_difficulty: u32,
    #[prost(uint32, tag = "5")]
    pub blocks: u32,
}
/// Generated server implementations.
pub mod chain_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ChainServiceServer.
    #[async_trait]
    pub trait ChainService: Send + Sync + 'static {
        async fn current_head(
            &self,
            request: tonic::Request<::types::prelude::Empty>,
        ) -> Result<tonic::Response<super::CurrentHeadResponse>, tonic::Status>;
        async fn block_level(
            &self,
            request: tonic::Request<::types::prelude::Empty>,
        ) -> Result<tonic::Response<super::GetBlockNumberResponse>, tonic::Status>;
        async fn get_block_by_hash(
            &self,
            request: tonic::Request<super::GetBlockByHashRequest>,
        ) -> Result<tonic::Response<::types::prelude::Block>, tonic::Status>;
        async fn get_block_by_level(
            &self,
            request: tonic::Request<super::GetBlockByLevelRequest>,
        ) -> Result<tonic::Response<::types::prelude::Block>, tonic::Status>;
        async fn get_blocks(
            &self,
            request: tonic::Request<super::GetBlocksRequest>,
        ) -> Result<tonic::Response<super::GetBlocksResponse>, tonic::Status>;
        async fn get_blockchain_info(
            &self,
            request: tonic::Request<::types::prelude::Empty>,
        ) -> Result<tonic::Response<super::ChainInfo>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ChainServiceServer<T: ChainService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ChainService> ChainServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ChainServiceServer<T>
    where
        T: ChainService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rpc.ChainService/CurrentHead" => {
                    #[allow(non_camel_case_types)]
                    struct CurrentHeadSvc<T: ChainService>(pub Arc<T>);
                    impl<T: ChainService> tonic::server::UnaryService<::types::prelude::Empty> for CurrentHeadSvc<T> {
                        type Response = super::CurrentHeadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::types::prelude::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).current_head(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CurrentHeadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.ChainService/BlockLevel" => {
                    #[allow(non_camel_case_types)]
                    struct BlockLevelSvc<T: ChainService>(pub Arc<T>);
                    impl<T: ChainService> tonic::server::UnaryService<::types::prelude::Empty> for BlockLevelSvc<T> {
                        type Response = super::GetBlockNumberResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::types::prelude::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).block_level(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlockLevelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.ChainService/GetBlockByHash" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByHashSvc<T: ChainService>(pub Arc<T>);
                    impl<T: ChainService> tonic::server::UnaryService<super::GetBlockByHashRequest>
                        for GetBlockByHashSvc<T>
                    {
                        type Response = ::types::prelude::Block;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockByHashRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_by_hash(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByHashSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.ChainService/GetBlockByLevel" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByLevelSvc<T: ChainService>(pub Arc<T>);
                    impl<T: ChainService> tonic::server::UnaryService<super::GetBlockByLevelRequest>
                        for GetBlockByLevelSvc<T>
                    {
                        type Response = ::types::prelude::Block;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockByLevelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_by_level(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByLevelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.ChainService/GetBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlocksSvc<T: ChainService>(pub Arc<T>);
                    impl<T: ChainService> tonic::server::UnaryService<super::GetBlocksRequest> for GetBlocksSvc<T> {
                        type Response = super::GetBlocksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlocksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_blocks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.ChainService/GetBlockchainInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockchainInfoSvc<T: ChainService>(pub Arc<T>);
                    impl<T: ChainService> tonic::server::UnaryService<::types::prelude::Empty>
                        for GetBlockchainInfoSvc<T>
                    {
                        type Response = super::ChainInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::types::prelude::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_blockchain_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockchainInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ChainService> Clone for ChainServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ChainService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ChainService> tonic::server::NamedService for ChainServiceServer<T> {
        const NAME: &'static str = "rpc.ChainService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Query {
    #[prost(message, optional, tag = "1")]
    pub app_id: ::core::option::Option<::primitive_types::Address>,
    #[prost(uint64, tag = "2")]
    pub service: u64,
    #[prost(uint64, tag = "3")]
    pub method: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub query: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDescriptorRequest {
    #[prost(message, optional, tag = "1")]
    pub app_id: ::core::option::Option<::primitive_types::Address>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDescriptorResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub descriptor: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStorage {
    #[prost(message, optional, tag = "1")]
    pub app_id: ::core::option::Option<::primitive_types::Address>,
    #[prost(bytes = "vec", tag = "2")]
    pub storage_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Generated server implementations.
pub mod runtime_api_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RuntimeApiServiceServer.
    #[async_trait]
    pub trait RuntimeApiService: Send + Sync + 'static {
        async fn query_runtime(
            &self,
            request: tonic::Request<::types::prelude::ApplicationCall>,
        ) -> Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        async fn query_runtime_storage(
            &self,
            request: tonic::Request<super::QueryStorage>,
        ) -> Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        async fn get_descriptor(
            &self,
            request: tonic::Request<super::GetDescriptorRequest>,
        ) -> Result<tonic::Response<super::GetDescriptorResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RuntimeApiServiceServer<T: RuntimeApiService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RuntimeApiService> RuntimeApiServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RuntimeApiServiceServer<T>
    where
        T: RuntimeApiService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rpc.RuntimeApiService/QueryRuntime" => {
                    #[allow(non_camel_case_types)]
                    struct QueryRuntimeSvc<T: RuntimeApiService>(pub Arc<T>);
                    impl<T: RuntimeApiService>
                        tonic::server::UnaryService<::types::prelude::ApplicationCall>
                        for QueryRuntimeSvc<T>
                    {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<::types::prelude::ApplicationCall>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_runtime(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryRuntimeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.RuntimeApiService/QueryRuntimeStorage" => {
                    #[allow(non_camel_case_types)]
                    struct QueryRuntimeStorageSvc<T: RuntimeApiService>(pub Arc<T>);
                    impl<T: RuntimeApiService> tonic::server::UnaryService<super::QueryStorage>
                        for QueryRuntimeStorageSvc<T>
                    {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryStorage>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_runtime_storage(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = QueryRuntimeStorageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.RuntimeApiService/GetDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetDescriptorSvc<T: RuntimeApiService>(pub Arc<T>);
                    impl<T: RuntimeApiService>
                        tonic::server::UnaryService<super::GetDescriptorRequest>
                        for GetDescriptorSvc<T>
                    {
                        type Response = super::GetDescriptorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_descriptor(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDescriptorSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RuntimeApiService> Clone for RuntimeApiServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RuntimeApiService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RuntimeApiService> tonic::server::NamedService for RuntimeApiServiceServer<T> {
        const NAME: &'static str = "rpc.RuntimeApiService";
    }
}
