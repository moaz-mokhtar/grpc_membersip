use self::members::{LicenseMemberRequest, LicenseMemberResponse};
use members::membership_server::Membership;
use members::{AddMemberRequest, AddMemberResponse};
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod members {
    tonic::include_proto!("members");
}

#[derive(Debug, Default)]
pub struct MembershipService {}

#[tonic::async_trait]
impl Membership for MembershipService {
    /// Add a new member
    /// *prams: AddMemberRequest
    async fn add_member(
        &self,
        request: Request<AddMemberRequest>,
    ) -> Result<Response<AddMemberResponse>, Status> {
        log::info!("Worker_Service.Membership.add_member: {:?}", request);

        let req = request.into_inner();

        let userid = Uuid::new_v4().to_string();
        let name = req.name;

        let reply = AddMemberResponse {
            successful: true,
            userid,
            name,
        };

        Ok(Response::new(reply))
    }

    /// License a member
    /// *prams: LicenseMemberRequest
    async fn license_member(
        &self,
        request: Request<LicenseMemberRequest>,
    ) -> Result<Response<LicenseMemberResponse>, Status> {
        log::info!("Worker_ServiceMembership.license_member: {:?}", request);

        let _req = request.into_inner();

        // Business logic to license/certify the member.
        let license_id = format!("LCN-123456789");

        let reply = LicenseMemberResponse {
            successful: true,
            license_id,
        };

        Ok(Response::new(reply))
    }
}
