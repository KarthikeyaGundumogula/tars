use serde::Deserialize;
use uuid::Uuid;

use crate::domain::{SetName, SetDescription, Statement};

#[derive(Deserialize)]
pub struct CreateSetReq {
    pub name: SetName,
    pub description: SetDescription,
    pub statement: Statement,
    pub profile_picture:String,
}

#[derive(Deserialize)]
pub struct JoinSetRequest {
    pub set_id: Uuid
}

#[derive(Deserialize)]
pub struct UpdateSetReq {
    pub name: Option<SetName>,
    pub description: Option<SetDescription>,
    pub statement: Option<Statement>,
    pub profile_picture: Option<String>,
}
