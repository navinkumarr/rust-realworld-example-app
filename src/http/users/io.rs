use core::types::user::User;
use core::types::io::get_current_user::*;
use rocket::{Data, Outcome, Request};
use rocket::http::Status;
use rocket::data::{self, FromData};
use rocket_contrib::Json;

use rocket::response::{Responder, Response};
use http::errors::io::*;
use chrono::Local;
use rocket::response;
use rocket;
use serde_json;
use http::api::ApiResult;

//@TODO: Remove unnecessary derive from entire codebase
#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentUserResponse {
    pub user: User,
}

impl<'r> Responder<'r> for ApiResult<CurrentUserOutput, CurrentUserError> {
    fn respond_to(self, req: &Request) -> Result<rocket::Response<'r>, Status> {
        let date = Local::now();

        let mut build = Response::build();

        match self.0 {
            Ok(output) => {
                build
                    .merge(response::content::Json(serde_json::to_string(&output)).respond_to(req)?);
                build.status(Status::Ok).ok()
            }
            Err(err) => {
                let err_response = match err {
                    CurrentUserError::RepoError(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("repo_error"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("RepoError"),
                        },
                    },
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}
