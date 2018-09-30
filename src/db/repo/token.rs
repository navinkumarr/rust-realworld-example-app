use jwt::{Algorithm, encode};
use chrono::Local;
use core::types::user::*;
use core::types::token::*;
use db::pool::*;
use core::types::error::RepoError;
use settings::Auth;

pub struct MysqlTokenRepo<'a> {
    db_conn: &'a DbConn,
    settings: &'a Auth,
}

impl<'a> MysqlTokenRepo<'a> {
    pub fn new(conn: &'a DbConn, settings: &'a Auth) -> MysqlTokenRepo<'a> {
        MysqlTokenRepo { db_conn: conn, settings }
    }
}

impl<'a> TokenRepo for MysqlTokenRepo<'a> {
    fn create_login_token(
        &self,
        user: &User,
    ) -> Result<String, RepoError> {
        let payload = json!({
            "iss" : self.settings.issuer,
            "sub" : user.username,
            "iat" : Local::now().timestamp(),
            "exp" : Local::now().timestamp() + self.settings.expiry
        });

        let header = json!({});
        encode(header, &self.settings.secret, &payload, Algorithm::HS256)
            .map_err(|e| RepoError::from(e))
    }

}

