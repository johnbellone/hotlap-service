// SPDX-License-Identifier: Apache-2.0
use pb::{DriverRequest, DriverResponse};
use tonic::{transport::Server, Code, Request, Response, Status};
use tonic_types::{ErrorDetails, StatusExt};

pub mod pb {
    tonic::include_proto!("hotlap.v1alpha.drivers");
}

#[derive(Debug, Default)]
pub struct DriverService;

#[tonic::async_trait]
impl pb::hotlap_v1alpha::DriverService for DriverService {
    #[tracing::instrument]
    async fn read(&self, req: Request<DriverRequest>) -> ServerResult<DriverResponse> {
        Err(Status::not_implemented())
    }
}
