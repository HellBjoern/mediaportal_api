use actix_web::{post, web, Responder, HttpResponse, http::StatusCode};
use mysql::{Pool, prelude::Queryable, params};

use crate::{utility::{checkname, logged}, structs::{User, Login, Username}};

/*
* User services
*/
#[post("/user/add")]
async fn adduser(params: web::Json<User>) -> impl Responder {
    let pool = match  Pool::new(crate::SQL) {
        Ok(pret) => pret,
        Err(err) => {
            println!("Could not create Pool; Error:\n{:?}", err);
            return HttpResponse::new(StatusCode::from_u16(452).unwrap());
        },
    };

    let mut conn = match pool.get_conn() {
        Ok(pooled_con) => pooled_con,
        Err(err) => {
            println!("Connection failed; Error:\n{:?}", err);
            return HttpResponse::new(StatusCode::from_u16(453).unwrap());
        },
    };

    match conn.exec_drop("INSERT INTO users(uusername, uemail, upassword) VALUES (?, ?, ?)", (&params.username, &params.email, &params.password)) {
        Ok(_) => return HttpResponse::new(StatusCode::from_u16(200).unwrap()),
        Err(_) => return HttpResponse::new(StatusCode::from_u16(454).unwrap()),
    };
}

#[post("/user/login")]
async fn login(valuser: web::Json<Login>) -> impl Responder {
    let pool = match  Pool::new(crate::SQL) {
        Ok(pret) => pret,
        Err(err) => {
            println!("Could not create Pool; Error:\n{:?}", err);
            return HttpResponse::new(StatusCode::from_u16(452).unwrap())
        },
    };

    let mut conn = match pool.get_conn() {
        Ok(pooled_con) => pooled_con,
        Err(err) => {
            println!("Connection failed; Error:\n{:?}", err);
            return HttpResponse::new(StatusCode::from_u16(453).unwrap());
        },
    };

    let res= match conn.exec_first("SELECT uusername, upassword FROM users WHERE uusername =:uname", params! { "uname" => &valuser.username}).map(|row| { row.map(|(uusername, upassword)| Login { username: uusername, password: upassword }) }) {
        Ok(ret) => ret,
        Err(_) => None,
    };
    if res.is_none() {
        return HttpResponse::new(StatusCode::from_u16(454).unwrap());
    }
    if res.unwrap().password == valuser.password {
        HttpResponse::new(StatusCode::from_u16(200).unwrap())
    } else {
        HttpResponse::new(StatusCode::from_u16(455).unwrap())
    }
}

#[post("/user/check")]
async fn check(username: web::Json<Username>) -> impl Responder {
    match checkname(username.username.to_string()) {
        Ok(res) => {
            if res {
                return HttpResponse::new(StatusCode::from_u16(200).unwrap());
            } else {
                return HttpResponse::new(StatusCode::from_u16(454).unwrap());
            }
        },
        Err(code) => return HttpResponse::new(StatusCode::from_u16(code).unwrap())
    };
}

#[post("/user/logged")]
async fn loggeds(username: web::Json<Username>) -> impl Responder {
    match logged(username.username.clone()) {
        Ok(res) => {
            if res {
                return HttpResponse::new(StatusCode::from_u16(200).unwrap());
            } else {
                return HttpResponse::new(StatusCode::from_u16(455).unwrap());
            }
        },
        Err(res) => return HttpResponse::new(StatusCode::from_u16(res).unwrap()),
    };
}