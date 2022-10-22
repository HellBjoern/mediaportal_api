use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header};
use log::info;
use crate::services::{user, data};
extern crate pretty_env_logger;

mod services;
mod other;

/*
* Constants
*/
//Hosting on Localhost
const IP: &str = "0.0.0.0";
//API Port
const PORT: u16 = 8080;
//Database connection
const SQL: &str = "mysql://user:password@127.0.0.1:3306/mediaportal";

/*
* Main Function
*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    info!("Starting api on {}:{}", IP, PORT);
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(user::add)
            .service(user::login)
            .service(user::logout)
            .service(user::chpwd)
            .service(user::check)
            .service(user::logged)
            .service(data::upload)
    })
    .bind((IP, PORT))?
    .run()
    .await
}