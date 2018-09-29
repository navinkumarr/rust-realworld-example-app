use std::{error, fmt};
use core::types::RepoError;
use std::error::Error as StdError;
use core::types::user::*;

#[derive(Debug, Deserialize)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUserOutput {
    pub user: UpdateUser,
}

#[derive(Debug, Serialize)]
pub enum UpdateUserError {
    InvalidInput(String),
    RepoError(String),
}

impl error::Error for UpdateUserError {
    fn description(&self) -> &str {
        match *self {
            UpdateUserError::InvalidInput(_) => "Invalid input.",
            UpdateUserError::RepoError(_) => "Something went wrong.",
        }
    }
}

impl fmt::Display for UpdateUserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateUserError::InvalidInput(ref err) => f.write_str(&err),
            UpdateUserError::RepoError(ref err) => f.write_str(&err),
        }
    }
}

impl From<RepoError> for UpdateUserError {
    fn from(e: RepoError) -> UpdateUserError {
        UpdateUserError::RepoError(e.description().to_string())
    }
}
