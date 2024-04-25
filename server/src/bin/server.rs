// SPDX-License-Identifier: Apache-2.0
use clap::Parser;
use dotenvy::dotenv;

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

    Server::builder()
        .trace_fn(|_| tracing::info_span!("hotlap_service_server"))
        .serve(args.address)
        .await?;

    Ok(())
}
