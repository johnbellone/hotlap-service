// SPDX-License-Identifier: Apache-2.0
use clap::Parser;
use hotlap_service_client::TokenInjector;
use hotlap_service_sdk::pb::{simulator_service_client::SimulatorServiceClient, SimulatorRequest};
use tonic::transport::Channel;
use tracing::debug;

#[derive(Parser, Debug)]
#[command(name = "hotlap-service-client")]
#[command(bin_name = "hotlap-service-client")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[clap(short = 'a', long = "address", default_value = "[::1]:50051")]
    address: std::net::SocketAddr,

    #[clap(value_parser = humantime::parse_duration, default_value = "5s")]
    timeout: std::time::Duration,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = Args::parse();

    let channel = Channel::from_static("http://[::1]:50051")
        .timeout(args.timeout)
        .rate_limit(5, std::time::Duration::from_secs(5))
        .concurrency_limit(256)
        .connect()
        .await?;

    // TODO: Implement JWT authentication/authorization here.
    let interceptor = TokenInjector::default();
    let mut client = SimulatorServiceClient::with_interceptor(channel, interceptor);

    let req = SimulatorRequest::default();
    debug!("req: {:?}", req);

    let res = client.read(req).await?;
    debug!("res: {:?}", res.into_inner());

    Ok(())
}
