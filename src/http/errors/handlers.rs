use rocket::Request;
use rocket_contrib::Json;
use rocket::Error;
use http::errors::io::*;
use chrono::Local;

#[catch(400)]
pub fn bad_request(_err: Error, req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 400,
            message: String::from("Incomplete Or Invalid Parameter Exception"),
            message_shortcode: String::from("incomplete_or_invalid_parameter"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            url: String::from(req.uri().as_str()),
            error_type: String::from("IncompleteOrInvalidParameterException"),
        },
    })
}

#[catch(401)]
pub fn unauthenticated(_err: Error, req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 401,
            message: String::from("Authorization Required"),
            message_shortcode: String::from("unauthenticated"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            url: String::from(req.uri().as_str()),
            error_type: String::from("AuthorizationRequired"),
        },
    })
}

#[catch(403)]
pub fn unauthorized(_err: Error, req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 403,
            message: String::from("Unauthorized"),
            message_shortcode: String::from("unauthorized"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            url: String::from(req.uri().as_str()),
            error_type: String::from("UnauthorizedException"),
        },
    })
}

#[catch(404)]
pub fn not_found(_err: Error, req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 404,
            message: String::from("Not Found"),
            message_shortcode: String::from("not_found"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            url: String::from(req.uri().as_str()),
            error_type: String::from("NotFound"),
        },
    })
}

#[catch(422)]
pub fn unprocessable_entity(_err: Error, req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    println!("{:?}", _err);
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 422,
            message: String::from("Unprocessable Entity"),
            message_shortcode: String::from("unprocessable_entity"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            url: String::from(req.uri().as_str()),
            error_type: String::from("UnprocessableEntity"),
        },
    })
}

#[catch(500)]
pub fn internal_server_error(_err: Error, req: &Request) -> Json<ErrorWrapper> {
    let date = Local::now();
    Json(ErrorWrapper {
        error: ErrorDetails {
            status: 500,
            message: String::from("Internal Server Error"),
            message_shortcode: String::from("internal_server_error"),
            datetime: date.format("%Y%m%d%H%M%S").to_string(),
            url: String::from(req.uri().as_str()),
            error_type: String::from("InternalServerError"),
        },
    })
}
