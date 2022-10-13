use actix_web::http::StatusCode;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::*;
use mysql::*;
use mysql::prelude::*;

#[derive(Deserialize, Debug)]
struct Request {
    text: String
}

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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 8080;
    println!("Starting api on {}:{}", ip, port);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(adduser)
            .service(login)
            .service(testpost)
    })
    .bind((ip, port))?
    .run()
    .await
}

#[post("/adduser")]
async fn adduser(params: web::Json<User>) -> impl Responder {
    let url = "mysql://user:password@127.0.0.1:3306/mediaportal";
    
    let pool = match  Pool::new(url) {
        Ok(pret) => pret,
        Err(_) => panic!("Pool broken"),
    };

    let mut conn = match pool.get_conn() {
        Ok(pooled_con) => pooled_con,
        Err(_) => panic!("Connection failed"),
    };

    match conn.exec_drop("INSERT INTO user(uusername, uemail, upassword) VALUES (?, ?, ?)", (&params.username, &params.email, &params.password)) {
        Ok(ret) => println!("{:?}", ret),
        Err(err) => panic!("{err}"),
    }
    println!("username: {}\nemail: {}\npassword: {}\n", params.username, params.email, params.password);
    HttpResponse::Ok()
}

#[post("/login")]
async fn login(params: web::Json<Login>) -> impl Responder {
    println!("username: {}, password: {}\n", params.username, params.password);
    HttpResponse::BadRequest()
}

//Testmethods - remove

#[derive(Deserialize)]
struct Test {
    value: String
}

#[post("/testpost")]
async fn testpost(params: web::Json<Test>) -> impl Responder {
    if params.value == "true" {
        HttpResponse::Ok().body("True")
    } else {
        HttpResponse::BadRequest().body("Hin")
    }
}

#[post("/")]
async fn hello(req: HttpRequest) -> impl Responder {
    let params = web::Query::<Request>::from_query(req.query_string()).unwrap();
    println!("{:?}", params.text);
    HttpResponse::Ok().body("ostia")
}