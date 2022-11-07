use actix_web::{post, Responder, web, HttpResponse};
use log::{warn, info, error};
use mysql::{prelude::Queryable, params};
use serde_json::json;
use pwhash::bcrypt;
use crate::other::{utility::{checkname_fn, get_conn_fn, logged_uname_fn, checkmail_fn}, structs::{Login, Username, User, Chpwd}};

//login service; takes json; responds with either code 400 on error + json msg or on success 200 + json msg
#[post("/user/login")]
async fn login(valuser: web::Json<Login>) -> impl Responder {
    info!("[REQ] /user/login");
    match checkname_fn(valuser.username.clone()) {
        Ok(res) => {
            if !res {
                warn!("attempted login with invalid credentials");
                return HttpResponse::BadRequest().json(json!({ "message":"Wrong Credentials!" }));
            }
        },
        Err(err) => {
            error!("checkname_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => {
            error!("get_conn_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    match conn.exec_first("SELECT uid, uusername, upassword FROM users WHERE uusername =:uname", params! { "uname" => &valuser.username}).map(|row| { row.map(|(uid, uusername, upassword)| Login { id: uid, username: uusername, password: upassword }) }) {
        Ok(res) => {
            if !res.is_none() && bcrypt::verify(valuser.password.clone(), &res.as_ref().unwrap().password) {
                match conn.exec_drop("UPDATE users SET ulogged = 1 WHERE uusername=:uname", params! { "uname" => &valuser.username}) {
                    Ok(_) => {
                        info!("successfully logged user in");
                        return HttpResponse::Ok().json(json!({ "id":&res.as_ref().unwrap().id, "username":res.as_ref().unwrap().username }))
                    },
                    Err(err) => {
                        error!("database threw error: {err}");
                        return HttpResponse::BadRequest().json(json!({"message":err.to_string()}))
                    },
                };
            } else {
                warn!("attempted login with invalid credentials");
                return HttpResponse::BadRequest().json(json!({ "message":"Wrong Credentials!" }));
            }
        },
        Err(err) => {
            error!("database threw error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }))
        },
    };
}

//logout user of database
#[post("/user/logout")]
async fn logout(username: web::Json<Username>) -> impl Responder {
    info!("[REQ] /user/logout");
    match checkname_fn(username.username.clone()) {
        Ok(res) => {
            if !res {
                warn!("tried logging out nonexistent user");
                return HttpResponse::BadRequest().json(json!({ "message":"User does not exist!" }));
            }
        },
        Err(err) => {
            error!("checkname_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => {
            error!("get_conn_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    match conn.exec_drop("UPDATE users SET ulogged = 0 WHERE uusername=:uname", params! { "uname" => &username.username}) {
        Ok(_) => {
            info!("successfully logged user out");
            return HttpResponse::Ok().json(json!({ "message":"Successfully logged out!" }))
        },
        Err(err) => {
            error!("database threw error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }))
        },
    };
}

//checks if user is logged in database; returns 200 + json msg on success or 400 + json msg on failure 
#[post("/user/logged")]
async fn logged(username: web::Json<Username>) -> impl Responder {
    info!("[REQ] /user/logged");
    match logged_uname_fn(username.username.clone()) {
        Ok(res) => {
            info!("successfully answered logged request");
            return HttpResponse::Ok().json(json!({ "logged":res }))
        },
        Err(err) => {
            error!("logged_fn failed with error {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };
}

//inserts user into DB; 
#[post("/user/add")]
async fn add(user: web::Json<User>) -> impl Responder {
    info!("[REQ] /user/add");
    match checkname_fn(user.username.clone()) {
        Ok(res) => {
            if res {
                warn!("tried creating already existing user");
                return HttpResponse::BadRequest().json(json!({ "message":"User already exists!" }));
            }
        },
        Err(err) => {
            error!("checkname_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    match checkmail_fn(user.email.clone()) {
        Ok(res) => {
            if res {
                warn!("tried creating user with already existing email");
                return HttpResponse::BadRequest().json(json!({ "message":"Email already taken!" }));
            }
        },
        Err(err) => {
            error!("checkmail_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => {
            error!("get_conn_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    match conn.exec_drop("INSERT INTO users(uusername, uemail, upassword) VALUES (?, ?, ?)", (&user.username, &user.email, bcrypt::hash(&user.password).unwrap())) {
        Ok(_) => {
            match conn.exec_first("SELECT uid, uusername, upassword FROM users WHERE uusername =:uname", params! { "uname" => &user.username}).map(|row| { row.map(|(uid, uusername, upassword)| Login { id: uid, username: uusername, password: upassword }) }) {
                Ok(res) => {
                    if !res.is_none() {
                        info!("successfully added user; logging in");
                        return HttpResponse::Ok().json(json!({ "id":&res.as_ref().unwrap().id, "username":res.as_ref().unwrap().username }));
                    } else {
                        error!("weird database error; shouldnt happen?");
                        return HttpResponse::BadRequest().json(json!({ "message":"User does not exist? Database magic broke" }));
                    }
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

//attempts to change password for user;
#[post("/user/chpwd")]
async fn chpwd(chpwd: web::Json<Chpwd>) -> impl Responder {
    info!("[REQ] /user/chpwd");
    match checkname_fn(chpwd.username.clone()) {
        Ok(res) => {
            if !res {
                warn!("attempted to change password for invalid user");
                return HttpResponse::BadRequest().json(json!({ "message":"User does not exist!" }));
            }
        },
        Err(err) => {
            error!("checkname_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => {
            error!("get_conn_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err }))
        },
    };

    match conn.exec_first("SELECT uid, uusername, upassword FROM users WHERE uusername =:uname", params! { "uname" => &chpwd.username}).map(|row| { row.map(|(uid, uusername, upassword)| Login { id: uid, username: uusername, password: upassword }) }) {
        Ok(res) => {
            if !res.is_none() && bcrypt::verify(chpwd.oldpwd.clone(), &res.as_ref().unwrap().password) {
                match conn.exec_drop("UPDATE users SET upassword =:npwd WHERE uusername=:uname", params! { "npwd" => bcrypt::hash(&chpwd.newpwd).unwrap(),  "uname" => &chpwd.username}) {
                    Ok(_) => {
                        info!("successfully changed password");
                        return HttpResponse::Ok().json(json!({ "message":"Changed password successfully!" }))
                    },
                    Err(err) => {
                        error!("database threw error: {err}");
                        return HttpResponse::BadRequest().json(json!({"message":err.to_string()}))
                    },
                };
            } else {
                warn!("attempted chpwd with invalid credentials");
                return HttpResponse::BadRequest().json(json!({ "message":"Old password invalid!" }));
            }
        },
        Err(err) => {
            error!("database threw error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }))
        },
    };

}

//checks if username exists in DB; (REMOVE FOR PRODUCTION)
#[post("/user/check")]
async fn check(username: web::Json<Username>) -> impl Responder {
    info!("[REQ] /user/check");
    match checkname_fn(username.username.to_string()) {
        Ok(res) => {
            info!("successfully answered check request");
            return HttpResponse::Ok().json(json!({ "message":res }));
        },
        Err(err) => {
            error!("checkname_fn failed with error: {err}");
            return HttpResponse::BadRequest().json(json!({ "message":err.to_string() }))
        },
    };
}