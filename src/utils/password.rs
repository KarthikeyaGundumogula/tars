use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::errors::ApiError;

pub fn get_password_hash(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string()) 
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<(),ApiError> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    let res = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)?;
    Ok(res)
}
