use serde::Deserialize;

use crate::domain::{SetName, SetDescription, Statement};

#[derive(Deserialize)]
pub struct CreateSetReq {
    pub name: SetName,
    pub description: SetDescription,
    pub statement: Statement,
    pub profile_picture:String,
}
