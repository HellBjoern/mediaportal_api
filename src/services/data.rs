use std::{fs, path::Path};
use actix_easy_multipart::extractor::MultipartForm;
use actix_web::{post, Responder, web, HttpResponse};
use log::{info, error, warn};
use mysql::{prelude::Queryable, params};
use serde_json::{json, Map};
use crate::{other::{structs::{FileUpload, Yt, Uid, Media, Down}, utility::{read_to_vec, get_conn_fn, logged_uid_fn, checkuid_fn}}, CONFIG};

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
    return HttpResponse::BadRequest().json(json!({ "code":"69", "message":"olls hin"}));
}

#[post("/data/medialist")]
async fn medialist(user: web::Json<Uid>) -> impl Responder {
    info!("[REQ] /data/medialist");
    match checkuid_fn(user.uid) {
        Ok(ok) => {
            if !ok {
                warn!("attempted medialist for invalid user");
                return HttpResponse::BadRequest().json(json!({ "message":"User does not exist!" }));
            }
        },
        Err(err) => {
            warn!("checkuid_fn failed; reason: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }));
        },
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => {
            error!("get_conn_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };


    let media = match conn.exec_map("SELECT mid, mname, mformat FROM media WHERE uid =:uid", params! {"uid" => user.uid }, |(mid, mname, mformat)| Media { mid, mname, mformat }) {
        Ok(ok) => ok,
        Err(err) => {
            error!("database threw error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }));
        }
    };


    let mut list = Map::new();
    let mut i = 0;
    for m in media {
        list.insert(i.to_string(), json!(m) );
        i += 1;
    }

    info!("successfully built list; sending");
    return HttpResponse::Ok().json(list);
}

#[post("/data/yt_dl")]
async fn yt_dl(down: web::Json<Yt>) -> impl Responder {
    info!("[REQ] /data/yt_dl");
    match logged_uid_fn(down.uid.clone()) {
        Ok(ret) => {
            if ret {
                info!("user ist logged in; continuing");
            } else {
                error!("user is not logged in; aborting download");
                return HttpResponse::BadRequest().json(json!({ "message":"User is not logged in" }))
            }
        },
        Err(err) => {
            error!("logged_uid_fn failed with reason: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }));
        }
    };

    let fpath: String;
    let file = match crate::other::utility::yt_dl(down.uri.clone(), down.format) {
        Ok(ok1) => {
            fpath = ok1.0[0].clone();
            match read_to_vec(fpath.clone()) {
                Ok(ok) => {
                    match fs::remove_dir_all(format!("{}/{}", CONFIG.dlpath.clone(), ok1.1)) {
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

    match conn.exec_drop("INSERT INTO media(uid, mmedia, mname, mformat) VALUES (?, ?, ?, ?)", (down.uid, &file, mname.clone(), down.format)) {
        Ok(_) => {
            match conn.query_first("SELECT mid FROM media WHERE mtimestamp = (SELECT MAX(mtimestamp) FROM media)").map(|row: Option<i32>| { row.unwrap() }) {
                Ok(ret) => {
                    info!("successfully downloaded video and uploaded to db; size was {}mb", (file.len()/(1024*1024)));
                    return HttpResponse::Ok().json(json!({ "mid":ret, "message":format!("Successfully downloaded {}", down.uri), "filename":mname }))
                },
                Err(err) => {
                    error!("database threw error: {err}");
                    return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }))
                },
            };
        },
        Err(err) => {
            error!("database threw error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }))
        },
    };
}

#[post("/data/download")]
async fn download(down: web::Json<Down>) -> impl Responder {
    info!("[REQ] /data/download");
    match logged_uid_fn(down.uid.clone()) {
        Ok(ret) => {
            if ret {
                info!("user ist logged in; continuing");
            } else {
                error!("user is not logged in; aborting download");
                return HttpResponse::BadRequest().json(json!({ "message":"User is not logged in" }))
            }
        },
        Err(err) => {
            error!("logged_uid_fn failed with reason: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }));
        }
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => {
            error!("get_conn_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    match conn.exec_first("SELECT mmedia FROM media WHERE uid =:uid AND mid =:mid", params! { "uid" => down.uid, "mid" => down.mid }).map(|row: Option<Vec<u8>>| { row }) {
        Ok(ret) => {
            match ret {
                Some(ret) => {
                    info!("successfully fetched video from db; size was {}mb", (ret.len()/(1024*1024)));
                    info!("attempting to send response");
                    return HttpResponse::Ok().content_type("binary/octet-stream").body(ret)
                },
                None => {
                    warn!("tried retrieving invalid mid");
                    return HttpResponse::BadRequest().json(json!({ "message":"Invalid mid!"}));
                }
            }
            
        },
        Err(err) => {
            error!("database threw error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }))
        },
    };
}