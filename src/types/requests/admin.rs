use serde::Deserialize;
use crate::domain::{Handle, Password};

#[derive(Deserialize, Debug)]
pub struct AdminAuthRequest {
    pub admin_name: Handle,
    pub admin_password: Password,
}
