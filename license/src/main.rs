mod service;
mod utils;

use crate::service::{MembershipService, members::membership_server::MembershipServer};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::initiate_logging();
    println!("License Service have been started");

    let addr = "[::1]:50053".parse()?;

    let membership_service = MembershipService::default();

    Server::builder()
        .add_service(MembershipServer::new(membership_service))
        .serve(addr)
        .await?;

    Ok(())
}
