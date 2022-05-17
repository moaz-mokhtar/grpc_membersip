use tonic::{transport::Server, Request, Response, Status};

use members::membership_server::{Membership, MembershipServer};
use members::{AddMemberRequest, AddMemberResponse};

pub mod utils;
pub mod members {
    tonic::include_proto!("members");
}

#[derive(Debug, Default)]
pub struct MembershipService {}

#[tonic::async_trait]
impl Membership for MembershipService {
    async fn add_member(
        &self,
        request: Request<AddMemberRequest>,
    ) -> Result<Response<AddMemberResponse>, Status> {
        println!("Got a request [add_membership]: {:?}", request);

        let req = request.into_inner();

        let reply = AddMemberResponse {
            successful: true,
            message: format!("Member {} added.", req.name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::initiate_logging();
    println!("Server started");

    let addr = "[::1]:50051".parse()?;

    let membership_service = MembershipService::default();

    Server::builder()
        .add_service(MembershipServer::new(membership_service))
        .serve(addr)
        .await?;

    Ok(())
}
