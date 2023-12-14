use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{ Deserialize, Serialize };
use std::sync::Mutex;

mod todolist;            //todolist/mod folder
use todolist::services;  //todolist/services.rs

struct AppState {        //State of data
    todolist_entries: Mutex<Vec<TodolistEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64, 
    title: String
}

#[get("/")]
async fn index() -> String {
   "Heal Check".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![])
    });
    let host = "127.0.0.1";
    let port = 8080;
    println!("Server running on: {}:{}", host, port);
    println!("get: /todolist/entries");
    println!("post: /todolist/entries");
    println!("put: /todolist/entries/id");
    println!("delete: /todolist/entries/id");
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config) //todolist/services/config
            // .service(echo_post)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind((host, port))?
    .run()
    .await
}
