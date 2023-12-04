use std::net::SocketAddr;
mod boot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    boot::run_server(addr).await?;

    Ok(())
}
