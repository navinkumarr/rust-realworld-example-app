#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
extern crate config;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate frank_jwt as jwt;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate slugify;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod settings;
pub mod http;
pub mod core;
pub mod db;

use settings::Settings;
use log::Level;

pub fn init() {
    let settings = Settings::new().unwrap();
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, settings.log.filter.clone());
    env_logger::Builder::from_env(env).init();

    http::api::main(settings);
}
