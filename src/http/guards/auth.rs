use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};
use core::types::user::User;

use jwt::{Algorithm, decode};

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
        let access_token = request.headers().get_one("Authorization");
        let secret = String::from("realworld");

        match access_token {
            Some(token) => {
                let decoded = decode(&String::from(token), &secret, Algorithm::HS256);
                println!("{:?}", decoded);
                match decoded {
                    Ok((_header, _payload)) => Outcome::Success(User::new()),
                    Err(_err) => Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
