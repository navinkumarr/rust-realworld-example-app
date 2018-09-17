use std::{error, fmt};
use core::types::RepoError;
use std::error::Error as StdError;
use core::types::user::*;

#[derive(Debug, Deserialize)]
pub struct RegisterUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterUserOutput {
    pub user: NewUser,
}

#[derive(Debug, Serialize)]
pub enum RegisterUserError {
    InvalidInput(String),
    RepoError(String),
}

impl error::Error for RegisterUserError {
    fn description(&self) -> &str {
        match *self {
            RegisterUserError::InvalidInput(_) => "Invalid input.",
            RegisterUserError::RepoError(_) => "Something went wrong.",
        }
    }
}

impl fmt::Display for RegisterUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterUserError::InvalidInput(ref err) => f.write_str(&err),
            RegisterUserError::RepoError(ref err) => f.write_str(&err),
        }
    }
}

impl From<RepoError> for RegisterUserError {
    fn from(e: RepoError) -> RegisterUserError {
        RegisterUserError::RepoError(e.description().to_string())
    }
}
