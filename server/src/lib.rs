// SPDX-License-Identifier: Apache-2.0

use tonic::{
    metadata::{Ascii, AsciiMetadataValue, MetadataValue},
    Request, Status,
};
use tracing::debug;
use uuid::Uuid;

pub mod circuit;
pub mod driver;
pub mod event;
pub mod session;
pub mod simulator;
pub mod team;

pub type ServerResult<T> = Result<tonic::Response<T>, tonic::Status>;
pub type InterceptResult<T> = Result<tonic::Request<T>, tonic::Status>;

#[derive(Clone, Debug, Default)]
pub struct TraceInterceptor;

#[derive(Clone, Debug, Default)]
pub struct TokenInterceptor;

impl tonic::service::Interceptor for TraceInterceptor {
    #[tracing::instrument]
    fn call(&mut self, mut req: Request<()>) -> InterceptResult<()> {
        if !req.metadata().contains_key("x-trace-id") {
            let uuid: String = Uuid::now_v7().to_string();
            let value: MetadataValue<Ascii> = AsciiMetadataValue::try_from(uuid).unwrap();
            req.metadata_mut().insert("x-trace-id", value.clone());
            debug!("Trace-Id: {:?}", value);
        }

        Ok(req)
    }
}

impl tonic::service::Interceptor for TokenInterceptor {
    #[tracing::instrument]
    fn call(&mut self, req: Request<()>) -> InterceptResult<()> {
        match req.metadata().get("authorization") {
            Some(token) => {
                debug!("Authorization token: {:?}", token);
                Ok(req)
            }
            _ => Err(Status::unauthenticated("invalid token")),
        }
    }
}
