// SPDX-License-Identifier: Apache-2.0
use pb::{CircuitRequest, CircuitResponse};
use tonic::{transport::Server, Code, Request, Response, Status};
use tonic_types::{ErrorDetails, StatusExt};

pub mod pb {
    tonic::include_proto!("hotlap.v1alpha.circuits");
}

#[derive(Debug, Default)]
pub struct CircuitService;

#[tonic::async_trait]
impl pb::hotlap_v1alpha::CircuitService for CircuitService {
    #[tracing::instrument]
    async fn read(&self, req: Request<CircuitRequest>) -> ServerResult<CircuitResponse> {
        Err(Status::not_implemented())
    }
}
