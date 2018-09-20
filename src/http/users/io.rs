use core::types::user::User;
use core::types::io::get_current_user::*;
use core::types::io::register_user::*;
use core::types::io::login_user::*;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}

impl From<RegisterUserRequest> for RegisterUserInput {
    fn from(req: RegisterUserRequest) -> RegisterUserInput {
        RegisterUserInput {
            email: req.email,
            username: req.username,
            password: req.password,
        }
    }
}

impl From<LoginUserRequest> for LoginUserInput {
    fn from(req: LoginUserRequest) -> LoginUserInput {
        LoginUserInput {
            email: req.email,
            password: req.password,
        }
    }
}

impl FromData for RegisterUserInput {
    type Error = RegisterUserError;
    fn from_data(req: &Request, data: Data) -> data::Outcome<RegisterUserInput, Self::Error> {
        let unwrapped_json = Json::<RegisterUserRequest>::from_data(&req, data);

        if let Outcome::Failure(error) = unwrapped_json {
            let (_, serde_error) = error;

            return Outcome::Failure((
                Status::from_code(422).unwrap(),
                RegisterUserError::InvalidInput(serde_error.to_string()),
            ));
        }

        let json = unwrapped_json.unwrap().0;
        let payload = RegisterUserInput::from(json);
        Outcome::Success(payload)
    }
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
                    CurrentUserError::InvalidInput(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("invalid_input"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("IncompleteOrInvalidParameterException"),
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

impl FromData for LoginUserInput {
    type Error = LoginUserError;
    fn from_data(req: &Request, data: Data) -> data::Outcome<LoginUserInput, Self::Error> {
        let unwrapped_json = Json::<LoginUserRequest>::from_data(&req, data);

        if let Outcome::Failure(error) = unwrapped_json {
            let (_, serde_error) = error;

            return Outcome::Failure((
                Status::from_code(422).unwrap(),
                LoginUserError::InvalidInput(serde_error.to_string()),
            ));
        }

        let json = unwrapped_json.unwrap().0;
        let payload = LoginUserInput::from(json);
        Outcome::Success(payload)
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
                    RegisterUserError::RepoError(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("repo_error"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("RepoError"),
                        },
                    },
                    RegisterUserError::InvalidInput(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("invalid_input"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("IncompleteOrInvalidParameterException"),
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
                    LoginUserError::RepoError(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("repo_error"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("RepoError"),
                        },
                    },
                    LoginUserError::InvalidInput(message) => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from(message),
                            message_shortcode: String::from("invalid_input"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("IncompleteOrInvalidParameterException"),
                        },
                    },
                    LoginUserError::InvalidCredentials => ErrorWrapper {
                        error: ErrorDetails {
                            status: 400,
                            message: String::from("Invalid credentials"),
                            message_shortcode: String::from("invalid_credentials"),
                            datetime: date.format("%Y%m%d%H%M%S").to_string(),
                            url: String::from(req.uri().as_str()),
                            error_type: String::from("IncompleteOrInvalidParameterException"),
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
