## ACTIX-NOTES

### Basic Usage
rust-actix-web/Cargo.toml
```
[package]
name = "rust-actix-web"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4"
```
rust-actix-web/src/main.rs
```vim
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey There!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(hello)                  //async fn hello
            .service(echo)                   //async fn echo
            .route("/hey", web::get().to(manual_hello))     //async fn manual_hello
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```
Access in Browser
```
http://127.0.0.1:8080/hey
```
