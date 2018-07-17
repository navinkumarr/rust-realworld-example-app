use diesel::result::Error as DieselError;
use std::error::Error as StdError;
use core::types::RepoError;

mod models;
mod repo;
mod pool;
mod schema;

impl From<DieselError> for RepoError {
    fn from(e: DieselError) -> RepoError {
        RepoError::RepoError(e.description().to_string())
    }
}

pub use self::pool::*;
pub use self::repo::user::MysqlUserRepo;
