use std::error::Error as StdError;
use std::fmt;

pub mod user;
pub mod io;

#[derive(Debug, Serialize)]
pub enum RepoError {
    RepoError(String),
}

impl StdError for RepoError {
    fn description(&self) -> &str {
        match *self {
            RepoError::RepoError(ref s) => s,
        }
    }
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RepoError::RepoError(ref s) => f.write_str(&s),
        }
    }
}
