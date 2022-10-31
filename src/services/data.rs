use std::{fs, path::Path};
use actix_easy_multipart::extractor::MultipartForm;
use actix_web::{post, Responder, web, HttpResponse};
use log::{info, error, warn};
use mysql::{prelude::Queryable, params};
use serde_json::{json, Map};
use crate::{other::{structs::{FileUpload, Yt, Uid, Media, Down}, utility::{read_to_vec, get_conn_fn, logged_uid_fn, checkuid_fn, ffmpeg}}, CONFIG};

/*
* Data services
*/
#[post("/data/convert")]
async fn convert(form: MultipartForm<FileUpload>) -> impl Responder {
    info!("[REQ] /data/convert");
    match logged_uid_fn(form.uid.clone()) {
        Ok(ret) => {
            if ret {
                info!("user ist logged in; continuing");
            } else {
                error!("user is not logged in; aborting upload");
                return HttpResponse::BadRequest().json(json!({ "message":"User is not logged in" }))
            }
        },
        Err(err) => {
            error!("logged_uid_fn failed with reason: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }));
        }
    };
    
    let convpath = match ffmpeg(form.format, form.file.file.path().to_str().unwrap().to_string(), form.file.filename.as_ref().unwrap().to_string()) {
        Ok(ok) => ok,
        Err(err) => {
            error!("ffmpeg conversion failed; reason: {}", err);
            return HttpResponse::BadRequest().json(json!({ "message":err }));
        }
    };

    let outfname = Path::new(&convpath).file_name().unwrap().to_str().unwrap().chars().filter(|c| c.is_ascii()).collect::<String>();
    let fasvec = match read_to_vec(convpath.clone()) {
        Ok(ok) => {
            match fs::remove_dir_all(Path::new(&convpath).parent().unwrap()) {
                Ok(_) => {},
                Err(err) => error!("failed deleting file; reason: {}", err),
            };
            ok
        },
        Err(err) => {
            return HttpResponse::BadRequest().json(json!({ "message":format!("Failed reading file; Reason: {}", err)}))
        }
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => {
            error!("get_conn_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    match conn.exec_drop("INSERT INTO media(uid, mmedia, mname, mformat) VALUES (?, ?, ?, ?)", (form.uid, &fasvec, outfname.clone(), form.format)) {
        Ok(_) => {
            match conn.query_first("SELECT mid FROM media WHERE mtimestamp = (SELECT MAX(mtimestamp) FROM media)").map(|row: Option<i32>| { row.unwrap() }) {
                Ok(ret) => {
                    info!("successfully converted media and uploaded to db; size was {}mb", (fasvec.len()/(1024*1024)));
                    return HttpResponse::Ok().json(json!({ "mid":ret, "message":format!("Successfully converted to {}", outfname), "filename":outfname }))
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
                    match fs::remove_dir_all(format!("{}/{}", CONFIG.tmppath.clone(), ok1.1)) {
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