#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = "http://0.0.0.0:8000";
    let mut client = s3_iam::iampb::iam::iam_client::IamClient::connect(addr)
        .await
        .unwrap();

    let request = tonic::Request::new(s3_iam::iampb::iam::CreateUserRequest {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    });
    let resp = client.create_user(request).await.unwrap();
    println!("RESPONSE={:?}", resp);

    let request = tonic::Request::new(s3_iam::iampb::iam::CreateKeyRequest {
        name: "TestKey".to_string(),
        ..Default::default()
    });

    let resp = client.create_key(request).await.unwrap();
    println!("RESPONSE={:?}", resp);
}
