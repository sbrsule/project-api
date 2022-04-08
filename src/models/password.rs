use anyhow::Result;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_password(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let password = password.as_bytes();

    // Setup Argon2 with default params
    let argon2 = Argon2::default();

    // Hashes password into PHC string
    let password_hash = argon2.hash_password(password, &salt);
    Ok(password_hash.unwrap().to_string())
}

pub fn verify_password(password: &String, hashed_password: String) -> bool {
    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()

}
