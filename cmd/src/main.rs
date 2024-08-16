mod router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = router::Router::build().await?;

    // spawn actix web server
    let server_task = tokio::spawn(router.run_until_stopped());

    // spawn admin server

    // spawn grpc server

    // wait for all tasks to complete
    tokio::select! {
        o = server_task => {
            if let Err(e) = o {
                return Err(e.into());
            }
        }
    }
    Ok(())
}
