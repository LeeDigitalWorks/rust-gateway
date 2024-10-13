use tonic::transport::Server;

use s3_iam::iampb;

#[derive(Debug, Default)]
pub struct S3IAMServer {}

pub async fn start_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    Server::builder()
        .add_service(iampb::iam::iam_server::IamServer::new(S3IAMServer {}))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}

#[tonic::async_trait]
impl iampb::iam::iam_server::Iam for S3IAMServer {
    async fn create_user(
        &self,
        request: tonic::Request<iampb::iam::CreateUserRequest>,
    ) -> Result<tonic::Response<iampb::iam::CreateUserResponse>, tonic::Status> {
        // Implement your logic here
        unimplemented!()
    }

    async fn get_user(
        &self,
        request: tonic::Request<iampb::iam::GetUserRequest>,
    ) -> Result<tonic::Response<iampb::iam::GetUserResponse>, tonic::Status> {
        // Implement your logic here
        unimplemented!()
    }

    async fn delete_user(
        &self,
        request: tonic::Request<iampb::iam::DeleteUserRequest>,
    ) -> Result<tonic::Response<iampb::iam::DeleteUserResponse>, tonic::Status> {
        // Implement your logic here
        unimplemented!()
    }

    async fn create_key(
        &self,
        request: tonic::Request<iampb::iam::CreateKeyRequest>,
    ) -> Result<tonic::Response<iampb::iam::CreateKeyResponse>, tonic::Status> {
        // Implement your logic here
        unimplemented!()
    }

    async fn delete_key(
        &self,
        request: tonic::Request<iampb::iam::DeleteKeyRequest>,
    ) -> Result<tonic::Response<iampb::iam::DeleteKeyResponse>, tonic::Status> {
        // Implement your logic here
        unimplemented!()
    }

    type StreamKeysStream = tonic::codec::Streaming<iampb::iam::StreamKeysResponse>;

    async fn stream_keys(
        &self,
        request: tonic::Request<iampb::iam::StreamKeysRequest>,
    ) -> Result<tonic::Response<Self::StreamKeysStream>, tonic::Status> {
        // Implement your logic here
        unimplemented!()
    }
}
