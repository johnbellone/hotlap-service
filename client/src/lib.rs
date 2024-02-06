// SPDX-License-Identifier: Apache-2.0
use tonic::{
    metadata::{Ascii, AsciiMetadataValue, MetadataValue},
    Request, Status,
};
use tracing::debug;

type InterceptResult<T> = Result<Request<T>, Status>;

#[derive(Clone, Debug, Default)]
pub struct TokenInterceptor;

impl tonic::service::Interceptor for TokenInterceptor {
    #[tracing::instrument]
    fn call(&mut self, mut req: Request<()>) -> InterceptResult<()> {
        if !req.metadata().contains_key("authorization") {
            let token: String = "Bearer deadbeef".to_string();
            let value: MetadataValue<Ascii> = AsciiMetadataValue::try_from(token).unwrap();
            req.metadata_mut().insert("authorization", value.clone());
            debug!("Authorization token: {:?}", value);
        }

        Ok(req)
    }
}
