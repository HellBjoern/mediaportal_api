use actix_web::{post, web, Responder, HttpResponse, http::StatusCode};
use mysql::{Pool, prelude::Queryable, params};
use crate::other::{structs::{User, Login, Username}, utility::{checkname, logged}};

/*
* User services
*/
//add user
#[post("/user/add")]
async fn adduser(params: web::Json<User>) -> impl Responder {
    match checkname(params.username.clone()) {
        Ok(res) => {
            if res {
                return HttpResponse::new(StatusCode::from_u16(454).unwrap());
            }
        },
        Err(code) => return HttpResponse::new(StatusCode::from_u16(code).unwrap())
    };

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
        Err(_) => return HttpResponse::new(StatusCode::from_u16(455).unwrap()),
    };
}
//login user + set ulogged to 1
#[post("/user/login")]
async fn login(valuser: web::Json<Login>) -> impl Responder {
    match checkname(valuser.username.clone()) {
        Ok(res) => {
            if !res {
                return HttpResponse::new(StatusCode::from_u16(454).unwrap());
            }
        },
        Err(code) => return HttpResponse::new(StatusCode::from_u16(code).unwrap())
    };

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

    match conn.exec_first("SELECT uusername, upassword FROM users WHERE uusername =:uname", params! { "uname" => &valuser.username}).map(|row| { row.map(|(uusername, upassword)| Login { username: uusername, password: upassword }) }) {
        Ok(res) => {
            if res.unwrap().password == valuser.password {
                match conn.exec_drop("UPDATE users SET ulogged = 1 WHERE uusername=:uname", params! { "uname" => &valuser.username}) {
                    Ok(_) => {},
                    Err(_) => { println!("test"); return HttpResponse::new(StatusCode::from_u16(455).unwrap())},
                };
                return HttpResponse::new(StatusCode::from_u16(200).unwrap());
            } else {
                return HttpResponse::new(StatusCode::from_u16(456).unwrap());
            }
        },
        Err(_) => return HttpResponse::new(StatusCode::from_u16(455).unwrap()),
    };
    
}
//does username exist in db
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
//is user ulogged set
#[post("/user/logged")]
async fn loggeds(username: web::Json<Username>) -> impl Responder {
    match logged(username.username.clone()) {
        Ok(res) => {
            if res {
                return HttpResponse::new(StatusCode::from_u16(200).unwrap());
            } else {
                return HttpResponse::new(StatusCode::from_u16(456).unwrap());
            }
        },
        Err(res) => return HttpResponse::new(StatusCode::from_u16(res).unwrap()),
    };
}
//set ulogged for user to 0
#[post("/user/logout")]
async fn logout(username: web::Json<Username>) -> impl Responder {
    match checkname(username.username.clone()) {
        Ok(res) => {
            if !res {
                return HttpResponse::new(StatusCode::from_u16(454).unwrap());
            }
        },
        Err(code) => return HttpResponse::new(StatusCode::from_u16(code).unwrap())
    };

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

    match conn.exec_drop("UPDATE users SET ulogged = 0 WHERE uusername=:uname", params! { "uname" => &username.username}) {
        Ok(_) => return HttpResponse::new(StatusCode::from_u16(200).unwrap()),
        Err(_) => return HttpResponse::new(StatusCode::from_u16(455).unwrap()),
    };
}