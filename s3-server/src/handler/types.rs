use axum::async_trait;

#[async_trait]
pub trait Handler: ReadActions + WriteActions {}

#[async_trait]
pub trait ReadActions {
    async fn get_bucket(&self) -> Result<(), ()>;
    async fn get_object(&self) -> Result<(), ()>;
    async fn list_objects(&self) -> Result<(), ()>;
    async fn list_object_versions(&self) -> Result<(), ()>;
    async fn list_parts(&self) -> Result<(), ()>;
    async fn get_multiparts(&self) -> Result<(), ()>;
    async fn list_multiparts(&self) -> Result<(), ()>;
}

#[async_trait]
pub trait WriteActions {
    async fn abort_multipart(&self) -> Result<(), ()>;
    async fn complete_multipart(&self) -> Result<(), ()>;
    async fn copy_object(&self) -> Result<(), ()>;
    async fn create_bucket(&self) -> Result<(), ()>;
    async fn create_multipart(&self) -> Result<(), ()>;
    async fn delete_bucket(&self) -> Result<(), ()>;
    async fn delete_object(&self) -> Result<(), ()>;
    async fn delete_objects(&self) -> Result<(), ()>;
    async fn put_object(&self) -> Result<(), ()>;
}

