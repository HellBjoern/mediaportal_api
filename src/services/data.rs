use std::{fs, path::Path};

use actix_easy_multipart::extractor::MultipartForm;
use actix_web::{post, Responder, web, HttpResponse};
use log::{info, warn, error};
use mysql::prelude::Queryable;
use serde_json::json;
use crate::{other::{structs::{FileUpload, Yt}, utility::{checkuid_fn, read_to_vec, get_conn_fn, logged_uid_fn}}, CONFIG};

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

    //return HttpResponse::new(StatusCode::from_u16(690).unwrap()).json();
    return actix_web::HttpResponse::BadRequest().json(json!({ "code":"69", "message":"olls hin"}));
}

#[post("/data/yt_dl")]
async fn yt_dl(down: web::Json<Yt>) -> impl Responder {
    info!("[REQ] /data/yt_dl");
    match checkuid_fn(down.uid.clone()) {
        Ok(res) => {
            if !res {
                warn!("attempted download with invalid uid");
                return HttpResponse::BadRequest().json(json!({ "message":"Wrong uid!" }));
            } else {
                match logged_uid_fn(down.uid.clone()) {
                    Ok(ret) => {
                        if ret {
                            info!("user logged in");
                        } else {
                            error!("user is not logged in; aborting download");
                            return HttpResponse::BadRequest().json(json!({ "message":"User is not logged in" }))
                        }
                    },
                    Err(err) => {
                        error!("logged_uid_fn failed with reason: {err}");
                        return HttpResponse::BadRequest().json(json!({ "message":format!("logged_uid_fn failed with reason: {}", err) }));
                    }
                }
            }
        },
        Err(err) => {
            error!("checkuid_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    let fpath: String;
    let file = match crate::other::utility::yt_dl(down.uri.clone(), down.format, down.uid) {
        Ok(ok) => {
            fpath = ok[0].clone();
            info!("successfully downloaded video with uri {}", &down.uri);
            match read_to_vec(ok[0].clone()) {
                Ok(ok) => {
                    match fs::remove_dir_all(CONFIG.dlpath.clone()) {
                        Ok(_) => {},
                        Err(err) => error!("failed deleting file; reason: {}", err),
                    }
                    ok
                },
                Err(err) => return HttpResponse::BadRequest().json(json!({ "message":format!("Failed reading file; Reason: {}", err)}))
            }
        },
        Err(err) => {
            error!("{}", err);
            return HttpResponse::BadRequest().json(json!({ "message":err }));
        }
    };
    let mname = Path::new(&fpath).file_name().unwrap().to_str().unwrap().chars().filter(|c| c.is_ascii()).collect::<String>();

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => {
            error!("get_conn_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    match conn.exec_drop("INSERT INTO media(uid, mmedia, mname, mformat) VALUES (?, ?, ?, ?)", (down.uid, file, mname.clone(), down.format)) {
        Ok(_) => {
            match conn.query_first("SELECT mid FROM media WHERE mtimestamp = (SELECT MAX(mtimestamp) FROM media)").map(|row: Option<i32>| { row.unwrap() }) {
                Ok(ret) => {
                    info!("successfully downloaded video and uploaded to db");
                    return HttpResponse::Ok().json(json!({ "mid":ret, "message":format!("Successfully downloaded {}", down.uri), "filename":mname }))
                },
                Err(err) => {
                    error!("database threw error: 1 {err}");
                    return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }))
                },
            };
        },
        Err(err) => {
            error!("database threw error: 2 {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }))
        },
    };
}