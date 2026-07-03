use crate::domain::{Handle, Password};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AdminAuthRequest {
    pub admin_name: Handle,
    pub admin_password: Password,
}
