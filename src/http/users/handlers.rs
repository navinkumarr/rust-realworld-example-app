use settings::Settings;
use rocket::State;
use http::api::ApiResult;
use core::types::user::User;
use core::usecases::get_current_user::*;
use core::types::io::get_current_user::*;

#[get("/", format = "application/json")]
fn current_user_handler(
    user: User,
    settings: State<Settings>,
) -> ApiResult<CurrentUserOutput, CurrentUserError> {
    println!("{:?}", user);
    ApiResult(get_current_user(
        user,
    ))
}
