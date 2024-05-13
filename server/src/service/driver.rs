// SPDX-License-Identifier: Apache-2.0

use crate::service::ServerResult;
use hotlap_service_sdk::pb::{driver_service_server::DriverService, DriverRequest, DriverResponse};
use tonic::{Request, Status};

#[derive(Debug, Default)]
pub struct DriverServer;

#[tonic::async_trait]
impl DriverService for DriverServer {
    #[tracing::instrument]
    async fn read(&self, _req: Request<DriverRequest>) -> ServerResult<DriverResponse> {
        Err(Status::unimplemented("rpc not implemented"))
    }
}
