use core::types::user::*;
use core::types::article::*;
use core::types::error::*;
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

// @TODO: Refactor this file

// -- Create article

impl FromData for CreateArticleInput {
    type Error = CreateArticleError;
    fn from_data(req: &Request, data: Data) -> data::Outcome<CreateArticleInput, Self::Error> {
        let unwrapped_json = Json::<CreateArticleInput>::from_data(&req, data);

        if let Outcome::Failure(error) = unwrapped_json {
            let (_, serde_error) = error;

            return Outcome::Failure((
                Status::from_code(422).unwrap(),
                CreateArticleError::InvalidInput(serde_error.to_string()),
            ));
        }

        let json = unwrapped_json.unwrap().0;
        Outcome::Success(json)
    }
}

impl<'r> Responder<'r> for ApiResult<CreateArticleOutput, CreateArticleError> {
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
                    CreateArticleError::RepoError(message) => ErrorWrapper::repo_error(message),
                    CreateArticleError::InvalidInput(message) => ErrorWrapper::invalid_input(message),
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}
