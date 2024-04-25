// SPDX-License-Identifier: Apache-2.0
use tonic::{
    metadata::{Ascii, AsciiMetadataValue, MetadataValue},
    Request, Status,
};
use tracing::debug;
use uuid::Uuid;

type InterceptResult<T> = Result<Request<T>, Status>;
type ServerResult<T> = Result<Response<T>, Status>;

#[derive(Clone, Debug, Default)]
pub struct TokenInterceptor;

#[derive(Clone, Debug, Default)]
pub struct TraceInterceptor;

impl tonic::service::Interceptor for TokenInterceptor {
    #[tracing::instrument]
    fn call(&mut self, req: Request<()>) -> InterceptResult<()> {
        match req.metadata().get("authorization") {
            Some(token) => {
                debug!("Authorization token: {:?}", token);
                Ok(req)
            }
            _ => Err(Status::unauthenticated("Invalid token")),
        }
    }
}

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
