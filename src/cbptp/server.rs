use tonic::{transport::Server, Request, Response, Status};

use cbptp::blockchain_server::{Blockchain, BlockchainServer};
use cbptp::{CbptpReply, CbptpRequest};

pub mod cbptp {
    tonic::include_proto!("cbptp");
}

#[derive(Default)]
pub struct CBPTPBlockchain {}

#[tonic::async_trait]
impl Blockchain for CBPTPBlockchain {
    async fn submit_transaction(
        &self,
        request: Request<CbptpRequest>,
    ) -> Result<Response<CbptpReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = cbptp::CbptpReply {
            message: format!("version is  {}!", request.into_inner().version),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let cbptp_server = CBPTPBlockchain::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(BlockchainServer::new(cbptp_server))
        .serve(addr)
        .await?;

    Ok(())
}
