use core::types::user::*;
use diesel::prelude::*;
use db::models::user::*;
use db::pool::*;
use core::types::RepoError;
use db::schema::*;
use diesel;
use chrono::Local;

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
                token : m.token.unwrap_or(String::from("")),
                bio : m.bio.unwrap_or(String::from("")),
                image : m.image.unwrap_or(String::from("")),
            }))
        } else {
            Ok(None)
        }
    }

    fn save_new_user(
        &self,
        new_user: &NewUser,
    ) -> Result<usize, RepoError> {
        let date = Local::now();
        let date_time = date.format("%Y%m%d%H%M%S")
            .to_string()
            .parse::<u64>()
            .unwrap();

        let data_users = InsertUser {
            username: new_user.username.clone(),
            email: new_user.email.clone(),
            password: new_user.password.clone(),
            created_at: date_time,
            updated_at: date_time
        };

        let result_users = diesel::insert_into(users::table)
            .values(&data_users)
            .execute(&*self.db_conn.master)?;

        Ok(result_users)
    }
}
