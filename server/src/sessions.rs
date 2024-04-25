// SPDX-License-Identifier: Apache-2.0
use pb::{SessionRequest, SessionResponse};
use tonic::{transport::Server, Code, Request, Response, Status};
use tonic_types::{ErrorDetails, StatusExt};

pub mod pb {
    tonic::include_proto!("hotlap.v1alpha.sessions");
}

#[derive(Debug, Default)]
pub struct SessionService;

#[tonic::async_trait]
impl pb::hotlap_v1alpha::SessionService for SessionService {
    #[tracing::instrument]
    async fn read(&self, req: Request<SessionRequest>) -> ServerResult<SessionResponse> {
        Err(Status::not_implemented())
    }
}
