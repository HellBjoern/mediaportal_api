use actix_web::{post, Responder, web, HttpResponse};
use mysql::{prelude::Queryable, params};
use serde_json::json;
use crate::other::{utility::{checkname_fn, get_conn_fn, logged_fn}, structs::{Login, Username, User}};

//login service; takes json; responds with either code 400 on error + json msg or on success 200 + json msg
#[post("/user/login")]
async fn login(valuser: web::Json<Login>) -> impl Responder {
    match checkname_fn(valuser.username.clone()) {
        Ok(res) => {
            if !res {
                return HttpResponse::BadRequest().json(json!({ "message":"Wrong Credentials!" }));
            }
        },
        Err(err) => return HttpResponse::BadRequest().json(json!({ "message":err })),
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => return HttpResponse::BadRequest().json(json!({ "message":err })),
    };

    match conn.exec_first("SELECT uid, uusername, upassword FROM users WHERE uusername =:uname", params! { "uname" => &valuser.username}).map(|row| { row.map(|(uid, uusername, upassword)| Login { id: uid, username: uusername, password: upassword }) }) {
        Ok(res) => {
            if !res.is_none() && res.as_ref().unwrap().password == valuser.password {
                match conn.exec_drop("UPDATE users SET ulogged = 1 WHERE uusername=:uname", params! { "uname" => &valuser.username}) {
                    Ok(_) => return HttpResponse::Ok().json(json!({ "id":&res.as_ref().unwrap().id, "username":res.as_ref().unwrap().username })),
                    Err(err) => return HttpResponse::BadRequest().json(json!({"message":err.to_string()})),
                };
            } else {
                return HttpResponse::BadRequest().json(json!({ "message":"Wrong Credentials!" }));
            }
        },
        Err(_) => return HttpResponse::BadRequest().json(json!({ "message":"Database Error" })),
    };
}

//logout user of database
#[post("/user/logout")]
async fn logout(username: web::Json<Username>) -> impl Responder {
    match checkname_fn(username.username.clone()) {
        Ok(res) => {
            if !res {
                return HttpResponse::BadRequest().json(json!({ "message":"User does not exist!" }));
            }
        },
        Err(err) => return HttpResponse::BadRequest().json(json!({ "message":err })),
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => return HttpResponse::BadRequest().json(json!({ "message":err })),
    };

    match conn.exec_drop("UPDATE users SET ulogged = 0 WHERE uusername=:uname", params! { "uname" => &username.username}) {
        Ok(_) => return HttpResponse::Ok().json(json!({ "message":"Successfully logged out!" })),
        Err(err) => return HttpResponse::BadRequest().json(json!({ "message":err.to_string() })),
    };
}

//checks if user is logged in database; returns 200 + json msg on success or 400 + json msg on failure 
#[post("/user/logged")]
async fn logged(username: web::Json<Username>) -> impl Responder {
    match logged_fn(username.username.clone()) {
        Ok(res) => return HttpResponse::Ok().json(json!({ "logged":res })),
        Err(res) => return HttpResponse::BadRequest().json(json!({ "message":res })),
    };
}

//inserts user into DB; 
#[post("/user/add")]
async fn adduser(user: web::Json<User>) -> impl Responder {
    match checkname_fn(user.username.clone()) {
        Ok(res) => {
            if res {
                return HttpResponse::BadRequest().json(json!({ "message":"User already exists!" }));
            }
        },
        Err(code) => return HttpResponse::BadRequest().json(json!({ "message":code })),
    };

    let mut conn = match get_conn_fn() {
        Ok(conn) => conn,
        Err(err) => return HttpResponse::BadRequest().json(json!({ "message":err })),
    };

    match conn.exec_drop("INSERT INTO users(uusername, uemail, upassword) VALUES (?, ?, ?)", (&user.username, &user.email, &user.password)) {
        Ok(_) => {
            match conn.exec_first("SELECT uid, uusername, upassword FROM users WHERE uusername =:uname", params! { "uname" => &user.username}).map(|row| { row.map(|(uid, uusername, upassword)| Login { id: uid, username: uusername, password: upassword }) }) {
                Ok(res) => {
                    if !res.is_none() {
                        return HttpResponse::Ok().json(json!({ "id":&res.as_ref().unwrap().id, "username":res.as_ref().unwrap().username }));
                    } else {
                        return HttpResponse::BadRequest().json(json!({ "message":"User does not exist? Database magic broke" }));
                    }
                },
                Err(_) => return HttpResponse::BadRequest().json(json!({ "message":"Database Error" })),
            };
        },
        Err(err) => return HttpResponse::BadRequest().json(json!({ "message":err.to_string() })),
    };
}

//checks if username exists in DB
#[post("/user/check")]
async fn check(username: web::Json<Username>) -> impl Responder {
    match checkname_fn(username.username.to_string()) {
        Ok(res) => {
            return HttpResponse::Ok().json(json!({ "message":res }));
        },
        Err(err) => return HttpResponse::BadRequest().json(json!({ "message":err.to_string() })),
    };
}