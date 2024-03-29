#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountRequest {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountResponse {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyRequest {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyResponse {}
/// Request and Response messages for getKey
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyRequest {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// 32-byte public key
    #[prost(bytes = "vec", tag = "4")]
    pub ephemeral_encaps_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyResponse {
    /// 256-bit key, encapsulated
    #[prost(bytes = "vec", tag = "1")]
    pub key_ciphertext: ::prost::alloc::vec::Vec<u8>,
    /// ciphertext for key that encrypts key_ciphertext
    #[prost(bytes = "vec", tag = "2")]
    pub encaps_ciphertext: ::prost::alloc::vec::Vec<u8>,
}
/// Just for testing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pong {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod locky_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LockyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LockyClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> LockyClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> LockyClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            LockyClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Locky/CreateAccount");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Locky", "CreateAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateKeyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Locky/CreateKey");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Locky", "CreateKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::GetKeyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Locky/GetKey");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Locky", "GetKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn ping_pong(
            &mut self,
            request: impl tonic::IntoRequest<super::Ping>,
        ) -> std::result::Result<tonic::Response<super::Pong>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Locky/PingPong");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("api.Locky", "PingPong"));
            self.inner.unary(req, path, codec).await
        }
    }
}
