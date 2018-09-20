use std::{error, fmt};
use core::types::RepoError;
use std::error::Error as StdError;
use core::types::user::*;

#[derive(Debug, Deserialize)]
pub struct LoginUserInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginUserOutput {
    pub user: User,
}

#[derive(Debug, Serialize)]
pub enum LoginUserError {
    InvalidInput(String),
    RepoError(String),
    InvalidCredentials,
}

impl error::Error for LoginUserError {
    fn description(&self) -> &str {
        match *self {
            LoginUserError::InvalidInput(_) => "Invalid input.",
            LoginUserError::RepoError(_) => "Something went wrong.",
            LoginUserError::InvalidCredentials => "Invalid credentials.",
        }
    }
}

impl fmt::Display for LoginUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LoginUserError::InvalidInput(ref err) => f.write_str(&err),
            LoginUserError::RepoError(ref err) => f.write_str(&err),
            LoginUserError::InvalidCredentials => f.write_str("Invalid credentials."),
        }
    }
}

impl From<RepoError> for LoginUserError {
    fn from(e: RepoError) -> LoginUserError {
        LoginUserError::RepoError(e.description().to_string())
    }
}
