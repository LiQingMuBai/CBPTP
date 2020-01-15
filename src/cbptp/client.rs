use cbptp::blockchain_client::BlockchainClient;
use cbptp::CbptpRequest;

pub mod cbptp {
    tonic::include_proto!("cbptp");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BlockchainClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CbptpRequest {
        from: "Tonic".into(),
        to: "Allen".into(),
        version: "1.0.0".into(),
        input: "input".into(),
        sign_data: "signData".into(),
        // from: "Tonic".into(),
        // from: "Tonic".into(),
        // from: "Tonic".into(),
        // from: "Tonic".into(),

    });

    let response = client.submit_transaction (request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
