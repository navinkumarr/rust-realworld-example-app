use settings::Settings;
use rocket::State;
use http::api::ApiResult;
use core::types::user::CurrentUser;
use core::usecases::create_article::*;
use core::types::article::*;
use core::types::error::*;
use db::{DbConn, MysqlUserRepo, MysqlArticleRepo};

#[post("/", format = "application/json", data = "<create_article_input>")]
fn create_article_handler(
    current_user: CurrentUser,
    create_article_input: Result<CreateArticleInput, CreateArticleError>,
    db: DbConn,
    _settings: State<Settings>,
) -> ApiResult<CreateArticleOutput, CreateArticleError> {

    ApiResult(create_article_input.and_then(|input| {
        let user_repo = MysqlUserRepo::new(&db);
        let article_repo = MysqlArticleRepo::new(&db);
        create_article(
            current_user,
            input,
            &user_repo,
            &article_repo,
        )
    }))

}
