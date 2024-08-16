use std::{net::SocketAddr, sync::Arc};

use actix_web::{dev::Server, App, HttpServer};

pub struct Router {
    server: Server,
}

impl Router {
    pub async fn build() -> Result<Self, std::io::Error> {
        println!("{:?}", helper::CONFIG.bind_api_address);
        let addr: SocketAddr = helper::CONFIG.bind_api_address.parse().expect("Invalid address");
        let object_layer = storage::Storage::new();
        let api_router = Arc::new(s3api::ObjectApiRouter::new(Box::new(object_layer)));

        let server = run(addr, api_router).await?;

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    addr: SocketAddr,
    api_router: Arc<s3api::ObjectApiRouter>,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Arc::clone(&api_router))
            .configure(s3api::register_routes)
    })
    .bind(addr)?
    .run();
    Ok(server)
}
