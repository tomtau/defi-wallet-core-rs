/// VestingAccounts stored in keeper
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VestingAccounts {
    /// addresses defines addresses of all the vesting accounts at genesis
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// SupplyRequest is the request type for the Query/TotalSupply RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplyRequest {}
/// SupplyResponse is the response type for the Query/TotalSupply RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplyResponse {
    /// supply is the supply of the coins
    #[prost(message, repeated, tag = "1")]
    pub supply: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Query defines the gRPC querier service."]
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    // Workaround, add feature manually, it could be fixed after https://github.com/hyperium/tonic/issues/491
    #[cfg(feature = "transport")]
    impl QueryClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " TotalSupply queries the total supply of all coins."]
        pub async fn total_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::SupplyRequest>,
        ) -> Result<tonic::Response<super::SupplyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/chainmain.supply.v1.Query/TotalSupply");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " LiquidSupply queries the liquid supply of all coins."]
        pub async fn liquid_supply(
            &mut self,
            request: impl tonic::IntoRequest<super::SupplyRequest>,
        ) -> Result<tonic::Response<super::SupplyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/chainmain.supply.v1.Query/LiquidSupply");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the capability module's genesis state.
/// TODO: currently left empty (for versioning),
/// later, it may include fields needed for custom capabilities
/// (subscriptions, vaultable accounts, ...)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {}
