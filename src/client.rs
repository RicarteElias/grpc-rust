use full_service::full_client::FullClient;
use full_service::FullRequest;

pub mod full_service {
    tonic::include_proto!("serving");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FullClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(FullRequest {
        client: "Tim".into(),
        mask: false,
        features: "vp1,vp2,vp3,vp4".into()
    });

    let response = client.request_full(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}