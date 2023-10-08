use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> String {
    let byte_password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(byte_password, &salt);
    return password_hash.unwrap().to_string();
}

pub async fn is_valid_password(stored_hash: &str, user_password: &str) -> bool {
    let byte_password = user_password.as_bytes();

    let parsed_hash = match PasswordHash::new(&stored_hash) {
        Ok(hash) => hash,
        Err(_e) => return false,
    };

    let verify = Argon2::default().verify_password(byte_password, &parsed_hash);

    if verify.is_ok() {
        println!("Password is valid");
        return true;
    } else {
        println!("Password is invalid");
        return false;
    }
}
