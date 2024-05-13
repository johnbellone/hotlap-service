// SPDX-License-Identifier: Apache-2.0

use crate::service::ServerResult;
use hotlap_service_sdk::pb::{
    session_service_server::SessionService, SessionRequest, SessionResponse,
};
use tonic::{Request, Status};

#[derive(Debug, Default)]
pub struct SessionServer;

#[tonic::async_trait]
impl SessionService for SessionServer {
    #[tracing::instrument]
    async fn read(&self, _req: Request<SessionRequest>) -> ServerResult<SessionResponse> {
        Err(Status::unimplemented("rpc not implemented"))
    }
}
