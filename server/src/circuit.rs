// SPDX-License-Identifier: Apache-2.0

use crate::ServerResult;
use hotlap_service_sdk::pb::{
    circuit_service_server::CircuitService, CircuitRequest, CircuitResponse,
};
use tonic::{Request, Status};

#[derive(Debug, Default)]
pub struct CircuitServer;

#[tonic::async_trait]
impl CircuitService for CircuitServer {
    #[tracing::instrument]
    async fn read(&self, _req: Request<CircuitRequest>) -> ServerResult<CircuitResponse> {
        Err(Status::unimplemented("rpc not implemented"))
    }
}
