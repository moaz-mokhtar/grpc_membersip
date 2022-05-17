use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    Error, HttpResponse, Responder,
};
use log::info;

use members::membership_client::MembershipClient;
use members::AddMemberRequest;

use crate::entity::MemberRequest;

pub mod members {
    tonic::include_proto!("members");
}

pub fn routes_config(config: &mut ServiceConfig) {
    config.service(health).service(register);
}

#[get("/")]
/// Route to test API functionality without any communcation with DB.
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[post("/register")]
pub async fn register(
    data: web::Json<MemberRequest>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    info!("pram of `register`: {:?}", data.0.clone());
    let name: String = data.0.name;
    info!("name: {:?}", name);

    let mut client = MembershipClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(AddMemberRequest { name });

    let response = client.add_member(request).await?;

    info!("Membership RESPONSE = {:?}", response);

    Ok(HttpResponse::Ok()
        .json("from `/register` API which used for registering new membership api"))
}
