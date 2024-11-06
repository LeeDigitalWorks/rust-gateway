use std::collections::HashMap;

use rand::{
    distributions::{Alphanumeric, Uniform},
    thread_rng, Rng,
};
use tokio::sync::RwLock;
use tonic::transport::Server;

use s3_iam::{google::protobuf::Timestamp, iampb};

#[derive(Debug, Default)]
pub struct S3IAMServer {
    // Map of usernames to Users
    pub users: RwLock<HashMap<String, iampb::iam::User>>,

    // Map of access keys to Keys
    pub keys: RwLock<HashMap<String, iampb::iam::Key>>,
}

impl S3IAMServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_keys(keys: HashMap<String, iampb::iam::Key>) -> Self {
        let mut server = Self::new();
        server.keys = RwLock::new(keys);
        server
    }
}

pub async fn start_server(
    addr: &str,
    server: S3IAMServer,
) -> Result<(), Box<dyn std::error::Error>> {
    Server::builder()
        .add_service(iampb::iam::iam_server::IamServer::new(server))
        .serve(addr.parse()?)
        .await?;
    Ok(())
}

/// Generates a 20-character AWS Access Key
fn generate_access_key() -> String {
    let key: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(18)
        .map(char::from)
        .collect();

    format!("RG{}", key.to_uppercase())
}

/// Generates a 40-character AWS Secret Key
fn generate_secret_key() -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
  abcdefghijklmnopqrstuvwxyz\
  0123456789+/";

    thread_rng()
        .sample_iter(Uniform::new_inclusive(0, charset.len() - 1))
        .take(40)
        .map(|i| charset.chars().nth(i).unwrap())
        .collect()
}

#[tonic::async_trait]
impl iampb::iam::iam_server::Iam for S3IAMServer {
    async fn get_key(
        &self,
        request: tonic::Request<iampb::iam::GetKeyRequest>,
    ) -> Result<tonic::Response<iampb::iam::GetKeyResponse>, tonic::Status> {
        tracing::debug!("GetKey: {:?}", request);
        let request = request.into_inner();
        let keys = self.keys.read().await;
        let key = keys.get(&request.access_key).cloned();
        match key {
            Some(key) => Ok(tonic::Response::new(iampb::iam::GetKeyResponse {
                key: Some(key),
            })),
            None => Err(tonic::Status::not_found("Key not found")),
        }
    }

    async fn list_keys(
        &self,
        _request: tonic::Request<iampb::iam::ListKeysRequest>,
    ) -> Result<tonic::Response<iampb::iam::ListKeysResponse>, tonic::Status> {
        let keys = self.keys.read().await;
        let keys: Vec<_> = keys.values().cloned().collect();
        Ok(tonic::Response::new(iampb::iam::ListKeysResponse { keys }))
    }

    async fn create_user(
        &self,
        request: tonic::Request<iampb::iam::CreateUserRequest>,
    ) -> Result<tonic::Response<iampb::iam::CreateUserResponse>, tonic::Status> {
        let request = request.into_inner();
        let id = thread_rng().gen::<u64>();
        let user = iampb::iam::User {
            id,
            name: request.name,
            email: request.email,
            created_at: Some(Timestamp {
                seconds: chrono::Utc::now().timestamp(),
                nanos: 0,
            }),
            keys: vec![],
        };
        let mut users = self.users.write().await;
        users.insert(id.to_string(), user.clone());
        Ok(tonic::Response::new(iampb::iam::CreateUserResponse {
            user: Some(user),
        }))
    }

    async fn get_user(
        &self,
        request: tonic::Request<iampb::iam::GetUserRequest>,
    ) -> Result<tonic::Response<iampb::iam::GetUserResponse>, tonic::Status> {
        let request = request.into_inner();
        let users = self.users.read().await;
        let user = users.get(&request.id.to_string()).cloned();
        match user {
            Some(user) => Ok(tonic::Response::new(iampb::iam::GetUserResponse {
                user: Some(user),
            })),
            None => Err(tonic::Status::not_found("User not found")),
        }
    }

    async fn delete_user(
        &self,
        request: tonic::Request<iampb::iam::DeleteUserRequest>,
    ) -> Result<tonic::Response<iampb::iam::DeleteUserResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut users = self.users.write().await;
        let user = users.remove(&request.id.to_string());
        match user {
            Some(_) => Ok(tonic::Response::new(iampb::iam::DeleteUserResponse {})),
            None => Err(tonic::Status::not_found("User not found")),
        }
    }

    async fn create_key(
        &self,
        request: tonic::Request<iampb::iam::CreateKeyRequest>,
    ) -> Result<tonic::Response<iampb::iam::CreateKeyResponse>, tonic::Status> {
        let request = request.into_inner();
        let access_key = generate_access_key();
        let secret_key = generate_secret_key();
        let key = iampb::iam::Key {
            id: 0,
            user_id: request.user_id,
            name: request.name,
            access_key: access_key.clone(),
            secret_key: secret_key.clone(),
            created_at: Some(Timestamp {
                seconds: chrono::Utc::now().timestamp(),
                nanos: 0,
            }),
            grants: vec![],
        };
        let mut keys = self.keys.write().await;
        keys.insert(access_key.clone(), key.clone());
        Ok(tonic::Response::new(iampb::iam::CreateKeyResponse {
            key: Some(key),
        }))
    }

    async fn delete_key(
        &self,
        request: tonic::Request<iampb::iam::DeleteKeyRequest>,
    ) -> Result<tonic::Response<iampb::iam::DeleteKeyResponse>, tonic::Status> {
        let request = request.into_inner();
        let mut keys = self.keys.write().await;
        let key = keys.remove(&request.access_key);
        match key {
            Some(_) => Ok(tonic::Response::new(iampb::iam::DeleteKeyResponse {})),
            None => Err(tonic::Status::not_found("Key not found")),
        }
    }

    type StreamKeysStream = tokio_stream::wrappers::ReceiverStream<
        Result<iampb::iam::StreamKeysResponse, tonic::Status>,
    >;

    async fn stream_keys(
        &self,
        _request: tonic::Request<iampb::iam::StreamKeysRequest>,
    ) -> Result<tonic::Response<Self::StreamKeysStream>, tonic::Status> {
        let keys: Vec<_> = self
            .keys
            .read()
            .await
            .values()
            .cloned()
            .map(|key| iampb::iam::StreamKeysResponse { key: Some(key) })
            .collect();

        let (tx, rx) = tokio::sync::mpsc::channel(4);
        tokio::spawn(async move {
            for key in keys {
                tx.send(Ok(key)).await.unwrap();
            }
        });

        Ok(tonic::Response::new(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        ))
    }
}
