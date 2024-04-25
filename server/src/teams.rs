// SPDX-License-Identifier: Apache-2.0
use pb::{TeamRequest, TeamResponse};
use tonic::{transport::Server, Code, Request, Response, Status};
use tonic_types::{ErrorDetails, StatusExt};

pub mod pb {
    tonic::include_proto!("hotlap.v1alpha.teams");
}

#[derive(Debug, Default)]
pub struct TeamService;

#[tonic::async_trait]
impl pb::hotlap_v1alpha::TeamService for TeamService {
    #[tracing::instrument]
    async fn read(&self, req: Request<TeamRequest>) -> ServerResult<TeamResponse> {
        Err(Status::not_implemented())
    }
}
