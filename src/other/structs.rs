use actix_easy_multipart::{FromMultipart, File};
use serde::Deserialize;

/*
* Structs
*/
#[derive(FromMultipart)]
pub struct FileUpload {
    pub description: Option<String>,
    pub file: File
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct Username {
    pub username: String
}