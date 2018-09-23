use settings::Settings;
use rocket::State;
use http::api::ApiResult;
use core::types::user::CurrentUser;
use core::usecases::get_current_user::*;
use core::usecases::register_user::*;
use core::usecases::login_user::*;
use core::types::io::get_current_user::*;
use core::types::io::register_user::*;
use core::types::io::login_user::*;
use db::{DbConn, MysqlUserRepo, MysqlTokenRepo};

#[get("/", format = "application/json")]
fn current_user_handler(
    current_user: CurrentUser,
    db: DbConn,
    _settings: State<Settings>,
) -> ApiResult<CurrentUserOutput, CurrentUserError> {
    println!("{:?}", current_user);
    let user_repo = MysqlUserRepo::new(&db);
    ApiResult(get_current_user(
        current_user,
        &user_repo,
    ))
}

#[post("/", format = "application/json", data = "<register_user_input>")]
fn register_user_handler(
    register_user_input: Result<RegisterUserInput, RegisterUserError>,
    db: DbConn,
    _settings: State<Settings>,
) -> ApiResult<RegisterUserOutput, RegisterUserError> {

    if let Err(err) = register_user_input {
        return ApiResult(Err(err));
    }

    let user_repo = MysqlUserRepo::new(&db);
    ApiResult(register_user(
        register_user_input.unwrap(),
        &user_repo,
    ))
}

#[post("/login", format = "application/json", data = "<login_user_input>")]
fn login_user_handler(
    login_user_input: Result<LoginUserInput, LoginUserError>,
    db: DbConn,
    settings: State<Settings>,
) -> ApiResult<LoginUserOutput, LoginUserError> {

    if let Err(err) = login_user_input {
        return ApiResult(Err(err));
    }

    let user_repo = MysqlUserRepo::new(&db);
    let token_repo = MysqlTokenRepo::new(&db, &settings.auth);

    ApiResult(login_user(
        login_user_input.unwrap(),
        &user_repo,
        &token_repo,
    ))
}
