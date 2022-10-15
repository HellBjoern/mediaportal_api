use actix_easy_multipart::{extractor::MultipartForm};
use actix_web::{post, Responder, http::StatusCode, HttpResponse};

use crate::structs::FileUpload;

/*
* Data services
*/
#[post("/data/upload")]
async fn upload(form: MultipartForm<FileUpload>) -> impl Responder {
    println!("File received is {:?}", form.file);
    if form.description.is_some() {
        println!("Has description: {:?}", form.description.as_ref().unwrap());
    } else {
        println!("Has no description");
    }
    return HttpResponse::new(StatusCode::from_u16(690).unwrap());
}