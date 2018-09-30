use core::types::user::User;
use core::types::error::RepoError;

pub trait TokenRepo {
    fn create_login_token(&self, user: &User) -> Result<String, RepoError>;
}
