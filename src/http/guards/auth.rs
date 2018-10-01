use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use core::types::user::CurrentUser;
use settings::Settings;

use jwt::{Algorithm, decode};

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<CurrentUser, ()> {
        let settings = request.guard::<State<Settings>>()?;
        let secret = settings.auth.secret.clone();

        request.headers().get_one("Authorization")
            .and_then(|token| {
                let token = if token.len() > 6 {&token[6..]} else {""}; // "Token JWTToken"
                println!("token {:?}", token);
                decode(&token.to_string(), &secret, Algorithm::HS256).ok()
            })
            .and_then(|(_header, payload)| {
                payload.get("sub")
                    .and_then(|sub| {
                        payload.get("id").and_then(|id| {
                            Some(Outcome::Success(CurrentUser{ 
                                username: String::from(sub.as_str().unwrap_or("")),
                                id: id.as_str().unwrap_or("0").parse::<u32>().unwrap(),
                            }))
                        })
                    })
            })
            .unwrap_or(Outcome::Failure((Status::Unauthorized, ())))

    }
}
