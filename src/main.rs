use std::{path::Path, fs};

use actix_cors::Cors;
use actix_easy_multipart::MultipartFormConfig;
use actix_web::{App, HttpServer, http::header};
use log::{info, warn};
use lazy_static::lazy_static;
use crate::{services::{user, data}, other::{structs::Config, utility::get_conf}};
extern crate pretty_env_logger;

mod services;
mod other;

//config location
const CONFPATH: &str = "api.toml";
//config struct
lazy_static! {
    pub static ref CONFIG: Config = get_conf();
}
//sql connection string
lazy_static! {
    pub static ref SQL: String = format!("mysql://{}:{}@{}:{}/{}", CONFIG.sqlusr, CONFIG.sqlpwd, CONFIG.sqladd, CONFIG.sqlprt, CONFIG.sqldab);
}

/*
* Main Function
*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    info!("Starting api on {}:{}", CONFIG.ip, CONFIG.port);
    info!("Cleaning up tmp dir");
    match fs::remove_dir_all(Path::new(&CONFIG.tmppath)) {
        Ok(_) => {},
        Err(err) => warn!("failed deleting temp folder; reason: {}; continuing...", err),
    };
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(MultipartFormConfig::default().total_limit(100 * 1024 * 1024 * 1024))
            .service(user::add)
            .service(user::login)
            .service(user::logout)
            .service(user::logged)
            .service(user::chpwd)
            .service(user::check)
            .service(data::convert)
            .service(data::yt_dl)
            .service(data::medialist)
            .service(data::download)
    })
    .bind((CONFIG.ip.clone(), CONFIG.port))?
    .run()
    .await
}
