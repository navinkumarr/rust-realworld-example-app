use core::types::user::*;
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

// -- Register user

impl FromData for RegisterUserInput {
    type Error = RegisterUserError;
    fn from_data(req: &Request, data: Data) -> data::Outcome<RegisterUserInput, Self::Error> {
        let unwrapped_json = Json::<RegisterUserInput>::from_data(&req, data);

        if let Outcome::Failure(error) = unwrapped_json {
            let (_, serde_error) = error;

            return Outcome::Failure((
                Status::from_code(422).unwrap(),
                RegisterUserError::InvalidInput(serde_error.to_string()),
            ));
        }

        let json = unwrapped_json.unwrap().0;
        Outcome::Success(json)
    }
}

impl<'r> Responder<'r> for ApiResult<RegisterUserOutput, RegisterUserError> {
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
                    RegisterUserError::RepoError(message) => ErrorWrapper::repo_error(message),
                    RegisterUserError::InvalidInput(message) => ErrorWrapper::invalid_input(message),
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}

// -- Current user

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
                    CurrentUserError::RepoError(message) => ErrorWrapper::repo_error(message),
                    CurrentUserError::InvalidInput(message) => ErrorWrapper::invalid_input(message),
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}

// -- Login user
impl FromData for LoginUserInput {
    type Error = LoginUserError;
    fn from_data(req: &Request, data: Data) -> data::Outcome<LoginUserInput, Self::Error> {
        let unwrapped_json = Json::<LoginUserInput>::from_data(&req, data);

        if let Outcome::Failure(error) = unwrapped_json {
            let (_, serde_error) = error;

            return Outcome::Failure((
                Status::from_code(422).unwrap(),
                LoginUserError::InvalidInput(serde_error.to_string()),
            ));
        }

        let json = unwrapped_json.unwrap().0;
        Outcome::Success(json)
    }
}


impl<'r> Responder<'r> for ApiResult<LoginUserOutput, LoginUserError> {
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
                    LoginUserError::RepoError(message) => ErrorWrapper::repo_error(message),
                    LoginUserError::InvalidInput(message) => ErrorWrapper::invalid_input(message),
                    LoginUserError::InvalidCredentials => ErrorWrapper::invalid_input(String::from("Invalid credentials")),
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}

// -- Update user

impl FromData for UpdateUserInput {
    type Error = UpdateUserError;
    fn from_data(req: &Request, data: Data) -> data::Outcome<UpdateUserInput, Self::Error> {
        let unwrapped_json = Json::<UpdateUserInput>::from_data(&req, data);

        if let Outcome::Failure(error) = unwrapped_json {
            let (_, serde_error) = error;

            return Outcome::Failure((
                Status::from_code(422).unwrap(),
                UpdateUserError::InvalidInput(serde_error.to_string()),
            ));
        }

        let json = unwrapped_json.unwrap().0;
        Outcome::Success(json)
    }
}

impl<'r> Responder<'r> for ApiResult<UpdateUserOutput, UpdateUserError> {
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
                    UpdateUserError::RepoError(message) => ErrorWrapper::repo_error(message),
                    UpdateUserError::InvalidInput(message) => ErrorWrapper::invalid_input(message)
                };
                build.merge(
                    response::content::Json(serde_json::to_string(&err_response)).respond_to(req)?,
                );
                build.status(Status::BadRequest).ok()
            }
        }
    }
}
