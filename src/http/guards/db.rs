use db::{DbConn, DbCollection};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<DbCollection>>()?;
        let db_pool = &pool.db_conn_pool;
        match db_pool.get() {
            Ok(conn) => Outcome::Success(DbConn {
                master: conn
            }),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
