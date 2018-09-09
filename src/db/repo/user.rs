use core::types::user::*;
use diesel::prelude::*;
use db::models::user::*;
use db::pool::*;
use core::types::RepoError;
use db::schema::*;

pub struct MysqlUserRepo<'a> {
    db_conn: &'a DbConn,
}

impl<'a> MysqlUserRepo<'a> {
    pub fn new(conn: &'a DbConn) -> MysqlUserRepo<'a> {
        MysqlUserRepo { db_conn: conn }
    }
}

impl<'a> UserRepo for MysqlUserRepo<'a> {
    fn find_user_by_email(
        &self,
        email: String,
    ) -> Result<Option<User>, RepoError> {
        let mut result_users = users::table
            .filter(users::email.eq(&email))
            .load::<QueryUser>(&*self.db_conn.master)?;

        if result_users.len() > 0 {
            let m = result_users.pop().unwrap();
            Ok(Some(User {
                username : m.username,
                email : m.email,
                token : m.token,
                bio : m.bio.unwrap_or(String::from("")),
                image : m.image,
            }))
        } else {
            Ok(None)
        }
    }

}
