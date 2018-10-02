use diesel::result::Error as DieselError;
use std::error::Error as StdError;
use jwt::Error as JWTError;
use core::types::error::RepoError;

mod models;
mod repo;
mod pool;
mod schema;

impl From<DieselError> for RepoError {
    fn from(e: DieselError) -> RepoError {
        RepoError::RepoError(e.description().to_string())
    }
}

impl From<JWTError> for RepoError {
    fn from(e: JWTError) -> RepoError {
        RepoError::RepoError(String::from("Failed creating token"))
    }
}

pub use self::pool::*;
pub use self::repo::user::MysqlUserRepo;
pub use self::repo::token::MysqlTokenRepo;
pub use self::repo::article::MysqlArticleRepo;
