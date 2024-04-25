// SPDX-License-Identifier: Apache-2.0
use pb::{EventRequest, EventResponse};
use tonic::{transport::Server, Code, Request, Response, Status};
use tonic_types::{ErrorDetails, StatusExt};

pub mod pb {
    tonic::include_proto!("hotlap.v1alpha.events");
}

#[derive(Debug, Default)]
pub struct EventService;

#[tonic::async_trait]
impl pb::hotlap_v1alpha::EventService for EventService {
    #[tracing::instrument]
    async fn read(&self, req: Request<EventRequest>) -> ServerResult<EventResponse> {
        Err(Status::not_implemented())
    }
}
