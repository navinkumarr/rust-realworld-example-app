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
                debug!("token {:?}", token);
                decode(&token.to_string(), &secret, Algorithm::HS256).ok()
            })
            .and_then(|(_header, payload)| {
                debug!("header {:?} payload {:?}", _header, payload);
                payload.get("sub")
                    .and_then(|sub| {
                        debug!("sub {:?}", sub);
                        payload.get("uid").and_then(|uid| {
                            debug!("id {:?}", uid);
                            Some(Outcome::Success(CurrentUser{ 
                                username: String::from(sub.as_str()?),
                                id: uid.as_u64()? as u32,
                            }))
                        })
                    })
            })
            .unwrap_or(Outcome::Failure((Status::Unauthorized, ())))

    }
}
