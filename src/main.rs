use actix_cors::Cors;
use actix_easy_multipart::MultipartFormConfig;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use lazy_static::lazy_static;
use log::{info, warn};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{fs, path::Path};

use crate::{
    other::{
        structs::Config,
        utility::{get_conf, shutdown},
    },
    services::{data, user},
};
extern crate pretty_env_logger;

mod other;
mod services;

//config location
const CONFPATH: &str = "api.toml";
//config struct
lazy_static! {
    pub static ref CONFIG: Config = get_conf();
}
//sql connection string
lazy_static! {
    pub static ref SQL: String = format!(
        "mysql://{}:{}@{}:{}/{}",
        CONFIG.sqlusr, CONFIG.sqlpwd, CONFIG.sqladd, CONFIG.sqlprt, CONFIG.sqldab
    );
}

/*
* Main Function
*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    ctrlc::set_handler(move || {
        shutdown();
    })
    .expect("Error setting interrupt handler");

    pretty_env_logger::init();
    info!("Starting api on {}:{}", CONFIG.ip, CONFIG.port);
    info!("Trying to clean up tmp dir");
    match fs::remove_dir_all(Path::new(&CONFIG.tmppath)) {
        Ok(_) => {}
        Err(err) => warn!(
            "failed deleting temp folder; reason: {}; continuing...",
            err
        ),
    };

    let mut continute = true;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    if CONFIG.ssl {
        match builder.set_private_key_file(CONFIG.key.clone(), SslFiletype::PEM) {
            Ok(_) => {
                info!("loaded key file; continuing...");
            }
            Err(_) => {
                warn!("could not load key file!");
                continute = false;
            }
        }

        match builder.set_certificate_chain_file(CONFIG.cert.clone()) {
            Ok(_) => {
                info!("loaded cert file; continuing...");
            }
            Err(_) => {
                warn!("could not load cert file!");
                continute = false;
            }
        }
    }

    if continute && CONFIG.ssl {
        info!("trying to start with ssl enabled...");

        HttpServer::new(move || {
            let logger = Logger::default();
            let cors = Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600);
            App::new()
                .wrap(cors)
                .wrap(logger)
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
        .workers(CONFIG.workers)
        .bind_openssl(format!("{}:{}", CONFIG.ip, CONFIG.port), builder)?
        .run()
        .await
    } else {
        info!("trying to start without ssl...");
        HttpServer::new(move || {
            let logger = Logger::default();
            let cors = Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600);
            App::new()
                .wrap(cors)
                .wrap(logger)
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
        .workers(CONFIG.workers)
        .bind((CONFIG.ip.clone(), CONFIG.port))?
        .run()
        .await
    }
}
