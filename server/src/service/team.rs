// SPDX-License-Identifier: Apache-2.0

use crate::service::ServerResult;
use hotlap_service_sdk::pb::{team_service_server::TeamService, TeamRequest, TeamResponse};
use tonic::{Request, Status};

#[derive(Debug, Default)]
pub struct TeamServer;

#[tonic::async_trait]
impl TeamService for TeamServer {
    #[tracing::instrument]
    async fn read(&self, _req: Request<TeamRequest>) -> ServerResult<TeamResponse> {
        Err(Status::unimplemented("rpc not implemented"))
    }
}
