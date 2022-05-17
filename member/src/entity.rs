// use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    // pub last_name: String,
    // pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberRequest {
    pub name: String,
    // pub last_name: String,
    // pub email: String,
    // pub password: String,
    // pub password_confirm: String,
}
