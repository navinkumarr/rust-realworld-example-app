use settings::Settings;
use rocket::State;
use http::api::ApiResult;
use core::types::user::CurrentUser;
use core::usecases::get_current_user::*;
use core::usecases::register_user::*;
use core::usecases::login_user::*;
use core::usecases::update_user::*;
use core::types::user::*;
use core::types::error::*;
use db::{DbConn, MysqlUserRepo, MysqlTokenRepo};

#[get("/", format = "application/json")]
fn current_user_handler(
    current_user: CurrentUser,
    db: DbConn,
    _settings: State<Settings>,
) -> ApiResult<CurrentUserOutput, CurrentUserError> {
    println!("{:?}", current_user);
    let user_repo = MysqlUserRepo::new(&db);
    let current_user_input = CurrentUserInput { user: current_user };
    ApiResult(get_current_user(
        current_user_input,
        &user_repo,
    ))
}

#[post("/", format = "application/json", data = "<register_user_input>")]
fn register_user_handler(
    register_user_input: Result<RegisterUserInput, RegisterUserError>,
    db: DbConn,
    _settings: State<Settings>,
) -> ApiResult<RegisterUserOutput, RegisterUserError> {

    ApiResult(register_user_input.and_then(|input| {
        let user_repo = MysqlUserRepo::new(&db);
        register_user(
            input,
            &user_repo,
        )
    }))

}

#[post("/login", format = "application/json", data = "<login_user_input>")]
fn login_user_handler(
    login_user_input: Result<LoginUserInput, LoginUserError>,
    db: DbConn,
    settings: State<Settings>,
) -> ApiResult<LoginUserOutput, LoginUserError> {

    ApiResult(login_user_input.and_then(|input| {
        let user_repo = MysqlUserRepo::new(&db);
        let token_repo = MysqlTokenRepo::new(&db, &settings.auth);

        login_user(
            input,
            &user_repo,
            &token_repo,
        )
    }))

}

#[put("/", format = "application/json", data = "<update_user_input>")]
fn update_user_handler(
    current_user: CurrentUser,
    update_user_input: Result<UpdateUserInput, UpdateUserError>,
    db: DbConn,
    _settings: State<Settings>,
) -> ApiResult<UpdateUserOutput, UpdateUserError> {

    ApiResult(update_user_input.and_then(|input| {
        let user_repo = MysqlUserRepo::new(&db);
        update_user(
            current_user,
            input,
            &user_repo,
        )
    }))

}
