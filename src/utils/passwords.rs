use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
        PasswordHash,
        Error
    },
    Argon2
};

// take a raw string slice and return the hashed password
pub fn hash_password(password: &str) -> Result<String, Error> {
    // generate a salt string using random noise from the os
    let salt = SaltString::generate(&mut OsRng);
    // declare the password hasher
    let argon2 = Argon2::default();
    // hash the password with the hash_password method (takes password as raw bytes and ref to salt string)
    let password_hash = argon2.hash_password(
        password.as_bytes(),
        &salt
    )?;
    Ok(password_hash.to_string())
}

// verify password against hash
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, Error> {
    // serialize the hash string into a PasswordHash struct
    let parsed_hash = PasswordHash::new(password_hash)?;
    // once more declare the argon2 hasher
    let argon2 = Argon2::default();
    // call the verify password method and if it doesn't return an error the hash is correct
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(Error::Password) => Ok(false),
        Err(_) => Ok(false)
    }
}