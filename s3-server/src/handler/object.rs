use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    filter::S3Data,
    server::{AppState, Server},
};

impl Server {
    pub async fn put_object(
        state: &Arc<AppState>,
        data: S3Data,
    ) -> axum::response::Response {
        unimplemented!()
    }

    pub async fn get_object(
        state: &Arc<AppState>,
        data: S3Data,
    ) -> axum::response::Response {
        unimplemented!()
    }
}
