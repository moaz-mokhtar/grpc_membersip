use self::members::{PrintLicenseRequest, PrintResponse};
use members::membership_server::Membership;
use tonic::{Request, Response, Status};

pub mod members {
    tonic::include_proto!("members");
}

#[derive(Debug, Default)]
pub struct MembershipService {}

#[tonic::async_trait]
impl Membership for MembershipService {
    /// Print a member's license
    /// *prams: PrintResponse
    async fn print_license(
        &self,
        request: Request<PrintLicenseRequest>,
    ) -> Result<Response<PrintResponse>, Status> {
        log::info!("license_Service.Membership.print_license: {:?}", request);

        let req = request.into_inner();

        let user_id = req.userid;
        let name = req.name;
        let license_id = req.license_id;

        // Generate the license as PDF
        generate_membership_license(user_id, name, license_id);

        let reply = PrintResponse { successful: true };

        Ok(Response::new(reply))
    }
}

fn generate_membership_license(user_id: String, name: String, license_id: String) {
    println!("*** Membership license PDF ***");
    let license_string = format!(
        "
    |\t\t MEMBERSHIP license \t\t|\n
    |\t\t USER ID: {} \t\t|\n
    |\t\t NAME: {} \t\t|\n
    |\t\t License ID: {} \t\t|\n
    ",
        user_id, name, license_id
    );

    println!("{}", license_string);
}
