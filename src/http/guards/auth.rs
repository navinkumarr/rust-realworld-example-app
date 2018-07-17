use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::collections::HashMap;
use core::types::user::User;

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let access_token = request.headers().get_one("X-Access-Token");

        match access_token {
            Some(token) => {
                Outcome::Success(User::new())
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
