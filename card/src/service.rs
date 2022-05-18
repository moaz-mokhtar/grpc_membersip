use self::members::{PrintCardRequest, PrintResponse};
use members::membership_server::Membership;
use tonic::{Request, Response, Status};

pub mod members {
    tonic::include_proto!("members");
}

#[derive(Debug, Default)]
pub struct MembershipService {}

#[tonic::async_trait]
impl Membership for MembershipService {
    /// Print a member's card information
    /// *prams: PrintResponse
    async fn print_card(
        &self,
        request: Request<PrintCardRequest>,
    ) -> Result<Response<PrintResponse>, Status> {
        log::info!("Card_Service.Membership.print_card: {:?}", request);

        let req = request.into_inner();

        let user_id = req.userid;
        let name = req.name;

        // Generate the card as PDF
        generate_membership_card(user_id, name);

        let reply = PrintResponse { successful: true };

        Ok(Response::new(reply))
    }
}

fn generate_membership_card(user_id: String, name: String) {
    println!("*** Membership Card PDF ***");
    let card_string = format!(
        "
    |\t\t MEMBERSHIP CARD \t\t|\n
    |\t\t USER ID: {} \t\t|\n
    |\t\t NAME: {} \t\t|\n
    ",
        user_id, name
    );
    println!("{}", card_string);
}
