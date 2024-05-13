// SPDX-License-Identifier: Apache-2.0

use hotlap_service_sdk::pb::{
    circuit_service_server::CircuitServiceServer, driver_service_server::DriverServiceServer,
    event_service_server::EventServiceServer, session_service_server::SessionServiceServer,
    simulator_service_server::SimulatorServiceServer, team_service_server::TeamServiceServer,
};
use hotlap_service_server::service::{
    circuit::CircuitServer,
    driver::DriverServer,
    event::EventServer,
    interceptor::{TokenInterceptor, TraceInterceptor},
    session::SessionServer,
    simulator::SimulatorServer,
    team::TeamServer,
};

use clap::Parser;
use dotenvy::dotenv;
use std::error::Error;
use std::time::Duration;
use tonic::transport::Server;

#[derive(Parser, Debug)]
#[command(name = "hotlap-service-server")]
#[command(bin_name = "hotlap-service-server")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[clap(short = 'a', long = "address", default_value = "[::1]:50051")]
    address: std::net::SocketAddr,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args: Args = Args::parse();

    // TODO: Add health service and reporter for production environment (non-debug).
    // TODO: Add reflect service for non-production environment (debug).

    let stack = tower::ServiceBuilder::new()
        .timeout(Duration::from_secs(30))
        .layer(tonic::service::interceptor(TraceInterceptor::default()))
        .layer(tonic::service::interceptor(TokenInterceptor::default()))
        .into_inner();

    let svc1 = CircuitServiceServer::new(CircuitServer::default());
    let svc2 = DriverServiceServer::new(DriverServer::default());
    let svc3 = EventServiceServer::new(EventServer::default());
    let svc4 = SessionServiceServer::new(SessionServer::default());
    let svc5 = SimulatorServiceServer::new(SimulatorServer::default());
    let svc6 = TeamServiceServer::new(TeamServer::default());

    Server::builder()
        .trace_fn(|_| tracing::info_span!("hotlap_service_server"))
        .layer(stack)
        .add_service(svc1)
        .add_service(svc2)
        .add_service(svc3)
        .add_service(svc4)
        .add_service(svc5)
        .add_service(svc6)
        .serve(args.address)
        .await?;

    Ok(())
}
