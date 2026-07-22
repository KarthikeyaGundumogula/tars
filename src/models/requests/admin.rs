use crate::domain::{Handle, Password, PermissionName, RoleName};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct AdminAuthRequest {
    pub admin_name: Handle,
    pub admin_password: Password,
}

#[derive(Deserialize, Debug)]
pub struct CreateRoleRequest {
    pub name: RoleName,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreatePermissionRequest {
    pub name: PermissionName,
    pub description: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AssignPermissionRequest {
    pub permission: PermissionName,
    pub role: RoleName,
}

#[derive(Deserialize, Debug)]
pub struct RevokePermissionRequest {
    pub permission: PermissionName,
    pub role: RoleName,
}

#[derive(Deserialize, Debug)]
pub struct UpdateProfileRoleReq {
    pub profile_id: Uuid,
    pub new_role: RoleName,
}