use std::{error, fmt};
use core::types::RepoError;
use std::error::Error as StdError;
use core::types::user::*;

#[derive(Debug, Deserialize)]
pub struct CurrentUserInput {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct CurrentUserOutput {
    pub user: User,
}

#[derive(Debug, Serialize)]
pub enum CurrentUserError {
    InvalidInput(String),
    RepoError(String),
}

impl error::Error for CurrentUserError {
    fn description(&self) -> &str {
        match *self {
            CurrentUserError::InvalidInput(_) => "Invalid input.",
            CurrentUserError::RepoError(_) => "Something went wrong.",
        }
    }
}

impl fmt::Display for CurrentUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CurrentUserError::InvalidInput(ref err) => f.write_str(&err),
            CurrentUserError::RepoError(ref err) => f.write_str(&err),
        }
    }
}

impl From<RepoError> for CurrentUserError {
    fn from(e: RepoError) -> CurrentUserError {
        CurrentUserError::RepoError(e.description().to_string())
    }
}
