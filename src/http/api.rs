use rocket;
use http::users::handlers::*;
use http::errors::handlers::*;
use settings::Settings;
use rocket::Rocket;

pub struct ApiResult<R, E>(pub Result<R, E>);

pub fn main(settings: Settings) {
    rocket(settings).launch();
}

pub fn rocket(settings: Settings) -> Rocket {
    rocket::ignite()
        .manage(settings)
        .mount("/api/user", routes![current_user_handler])
        .catch(catchers![
            not_found,
            unauthenticated,
            unauthorized,
            bad_request,
            unprocessable_entity,
            internal_server_error
        ])
}
