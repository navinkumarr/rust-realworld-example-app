extern crate rocket;
extern crate rust_realworld_example_app;

use rust_realworld_example_app::http::api::*;
use rust_realworld_example_app::settings::Settings;
use rocket::local::Client;
use rocket::http::Status;
use rocket::http::ContentType;

#[test]
fn login_user() {
    let settings = Settings::new().unwrap();
    let client = Client::new(rocket(settings)).expect("valid rocket instance");
    let data = r#"{
      "user": {
        "email": "navin.sing@gmail.com",
        "password": "navin"
      }
    }"#;
    let mut response = client
        .post("/api/users/login")
        .header(ContentType::JSON)
        .body(data)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    //assert_eq!(
        //response.body_string(),
        //Some(r#"{"status":"success"}"#.into())
    //);
}
