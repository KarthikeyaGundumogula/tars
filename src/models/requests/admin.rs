use crate::domain::{Handle, Password};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct AdminAuthRequest {
    pub admin_name: Handle,
    pub admin_password: Password,
}

#[derive(Deserialize, Debug)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreatePermissionRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AssignPermissionRequest {
    pub permission: String,
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct RevokePermissionRequest {
    pub permission: String,
    pub role: String,
}

#[derive(Deserialize,Debug)]
pub struct UpdateProfileRoleReq{
    pub profile_id: Uuid,
    pub new_role:String
}