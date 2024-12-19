// SPDX-License-Identifier: Apache-2.0

use crate::ServerResult;
use hotlap_service_sdk::pb::{event_service_server::EventService, EventRequest, EventResponse};
use tonic::{Request, Status};

#[derive(Debug, Default)]
pub struct EventServer;

#[tonic::async_trait]
impl EventService for EventServer {
    #[tracing::instrument]
    async fn read(&self, _req: Request<EventRequest>) -> ServerResult<EventResponse> {
        Err(Status::unimplemented("rpc not implemented"))
    }
}
