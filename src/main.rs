use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use mysql::{Pool, prelude::Queryable, params};
use serde::Deserialize;

//Hosting on Localhost
static IP: &str = "127.0.0.1";
//API Port
static PORT: u16 = 8080;
//Database connection
static SQL: &str = "mysql://user:password@127.0.0.1:3306/mediaportal";

#[derive(Deserialize)]
struct User {
    username: String,
    email: String,
    password: String
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String
}

#[derive(Deserialize)]
struct Username {
    username: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting api on {}:{}", IP, PORT);
    HttpServer::new(|| {
        App::new()
            .service(adduser)
            .service(login)
            .service(check)
    })
    .bind((IP, PORT))?
    .run()
    .await
}

#[post("/user/add")]
async fn adduser(params: web::Json<User>) -> impl Responder {
    let pool = match  Pool::new(SQL) {
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
    let pool = match  Pool::new(SQL) {
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
    let pool = match  Pool::new(SQL) {
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

    let res= match conn.exec_first("SELECT uusername, upassword FROM users WHERE uusername =:uname", params! { "uname" => &username.username}).map(|row| { row.map(|(uusername, upassword)| Login { username: uusername, password: upassword }) }) {
        Ok(ret) => ret,
        Err(_) => None,
    };
    if res.is_none() {
        return HttpResponse::new(StatusCode::from_u16(454).unwrap());
    } else {
        return HttpResponse::new(StatusCode::from_u16(200).unwrap());
    }
}