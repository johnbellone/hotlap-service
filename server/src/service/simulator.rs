// SPDX-License-Identifier: Apache-2.0

use crate::service::ServerResult;
use hotlap_service_sdk::pb::{
    simulator_service_server::SimulatorService, SimulatorRequest, SimulatorResponse,
};
use tonic::{Request, Status};

#[derive(Debug, Default)]
pub struct SimulatorServer;

#[tonic::async_trait]
impl SimulatorService for SimulatorServer {
    #[tracing::instrument]
    async fn read(&self, _req: Request<SimulatorRequest>) -> ServerResult<SimulatorResponse> {
        Err(Status::unimplemented("rpc not implemented"))
    }
}
