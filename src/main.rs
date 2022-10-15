use actix_web::{App, HttpServer};
use crate::services::{user, data};

mod services;
mod other;

/*
* Constants
*/
//Hosting on Localhost
const IP: &str = "127.0.0.1";
//API Port
const PORT: u16 = 8080;
//Database connection
const SQL: &str = "mysql://user:password@127.0.0.1:3306/mediaportal";

/*
* Main Function
*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting api on {}:{}", IP, PORT);
    HttpServer::new(|| {
        App::new()
            .service(user::adduser)
            .service(user::login)
            .service(user::check)
            .service(user::loggeds)
            .service(data::upload)
    })
    .bind((IP, PORT))?
    .run()
    .await
}