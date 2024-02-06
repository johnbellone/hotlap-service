// SPDX-License-Identifier: Apache-2.0
use clap::Parser;
use hotlap_service_server::TokenInterceptor;
use pb::{
    hotlap_service_server::{HotlapService, HotlapServiceServer},
    DatumRequest, DatumResponse,
};
use std::{error::Error, io::ErrorKind, pin::Pin};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod pb {
    tonic::include_proto!("hotlap_service");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("hotlap_service_descriptor");
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<DatumResponse, Status>> + Send>>;
type ServerResult<T> = Result<Response<T>, Status>;

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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = Args::parse();

    // TODO: Add health service and reporter for production environment (non-debug).

    let reflect_service = if args.debug {
        let s = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();
        Some(s)
    } else {
        None
    };

    let server = LocalHotlapServer::default();
    let interceptor = TokenInterceptor::default();
    let service = HotlapServiceServer::with_interceptor(server, interceptor);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("hotlap_service_server"))
        .add_optional_service(reflect_service)
        .add_service(service)
        .serve(args.address)
        .await?;

    Ok(())
}

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

#[derive(Debug, Default)]
pub struct LocalHotlapServer {}

#[tonic::async_trait]
impl HotlapService for LocalHotlapServer {
    type RecordStream = ResponseStream;

    #[tracing::instrument]
    async fn record(&self, req: Request<Streaming<DatumRequest>>) -> ServerResult<ResponseStream> {
        tracing::debug!("HotlapService/record");
        let mut in_stream = req.into_inner();
        let (tx, rx) = mpsc::channel(128);

        // this spawn here is required if you want to handle connection error.
        // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
        // will be drooped when connection error occurs and error will never be propagated
        // to mapped version of `in_stream`.
        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => tx
                        .send(Ok(DatumResponse { uuid: v.uuid }))
                        .await
                        .expect("working rx"),
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                tracing::error!("\tclient disconnected: broken pipe");
                                break;
                            }
                        }

                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break, // response was droped
                        }
                    }
                }
            }

            tracing::debug!("\tstream ended");
        });

        // echo just write the same data that was received
        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(out_stream) as ResponseStream))
    }
}
