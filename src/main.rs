use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::*;
use mysql::*;
use mysql::prelude::*;

static IP: &str = "127.0.0.1";
static PORT: u16 = 8080;
static SQL: &str = "mysql://user:password@127.0.0.1:3306/mediaportal";

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
    println!("Starting api on {}:{}", IP, PORT);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(adduser)
            .service(login)
            .service(testpost)
    })
    .bind((IP, PORT))?
    .run()
    .await
}

#[post("/adduser")]
async fn adduser(params: web::Json<User>) -> impl Responder {
    let pool = match  Pool::new(SQL) {
        Ok(pret) => pret,
        Err(_) => panic!("Pool broken"),
    };

    let mut conn = match pool.get_conn() {
        Ok(pooled_con) => pooled_con,
        Err(_) => panic!("Connection failed"),
    };

    match conn.exec_drop("INSERT INTO users(uusername, uemail, upassword) VALUES (?, ?, ?)", (&params.username, &params.email, &params.password)) {
        Ok(ret) => println!("{:?}", ret),
        Err(err) => panic!("{err}"),
    }
    println!("username: {}\nemail: {}\npassword: {}\n", params.username, params.email, params.password);
    HttpResponse::Ok()
}

#[post("/login")]
async fn login(valuser: web::Json<Login>) -> impl Responder {
    let pool = match  Pool::new(SQL) {
        Ok(pret) => pret,
        Err(_) => panic!("Pool broken"),
    };

    let mut conn = match pool.get_conn() {
        Ok(pooled_con) => pooled_con,
        Err(_) => panic!("Connection failed"),
    };

    match conn.exec("SELECT uusername, upassword FROM users WHERE username = ?", valuser.username) {
        Ok(ret) => println!("{:?}", ret),
        Err(err) => panic!("{err}"),
    }
    println!("username: {}, password: {}", valuser.username, valuser.password);
    //let dbuser = 
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