use actix_easy_multipart::{tempfile::Tempfile, text::Text, MultipartForm};
use serde::{Deserialize, Serialize};

#[derive(MultipartForm)]
pub struct FileUpload {
    pub file: Vec<Tempfile>,
    pub format: Text<i32>,
    pub uid: Text<i32>,
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct Login {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Username {
    pub username: String,
}

#[derive(Deserialize)]
pub struct Chpwd {
    pub username: String,
    pub oldpwd: String,
    pub newpwd: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub workers: usize,
    pub sqladd: String,
    pub sqlusr: String,
    pub sqlpwd: String,
    pub sqlprt: u16,
    pub sqldab: String,
    pub tmppath: String,
    pub ssl: bool,
    pub cert: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct Yt {
    pub uid: i32,
    pub uri: String,
    pub format: i32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Media {
    pub mid: i32,
    pub mname: String,
    pub mformat: i32,
}

#[derive(Deserialize)]
pub struct Uid {
    pub uid: i32,
}

#[derive(Deserialize)]
pub struct Down {
    pub uid: i32,
    pub mid: i32,
}
