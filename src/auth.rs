use argon2::{PasswordHash, PasswordVerifier, PasswordHasher};
use argon2::password_hash::{SaltString, Error, rand_core::OsRng};
use argon2::Argon2;
use rand::Rng;
use rand::distributions::Alphanumeric;

use crate::models::User;

#[derive(serde::Deserialize)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

// Used to verify a user's password and return a session token
pub fn authorize_user(user: &User, credentials: &Credentials) -> Result<String, Error> {
  // Hash the user's password using argon and propagate errors if this fails
  let password_hash = PasswordHash::new(&user.password)?;
  let argon2 = Argon2::default();
  // Get the provided password from the credentials
  let password = credentials.password.as_bytes();
  // compare the provided password with the stored password hash and propagate errors if this fails
  argon2.verify_password(password, &password_hash)?;

  // Generate a session token
  Ok(
    rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(128)
    .map(char::from)
    .collect()
  )
}

// Used to hash a given password and return it
pub fn hash_password(password: &str) -> Result<String, Error> {
  let salt = SaltString::generate(OsRng);
  let argon2 = Argon2::default();
  let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
  Ok(password_hash.to_string())
}