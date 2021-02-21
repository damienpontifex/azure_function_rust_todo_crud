use std::collections::HashMap;
use std::sync::Mutex;
use std::net::{Ipv4Addr, Ipv6Addr};
use serde::{Deserialize, Serialize};
use actix_web::{App, HttpResponse, HttpServer, delete, get, http, post, web};

#[derive(Deserialize, Serialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

struct Database {
    store: Mutex<HashMap<u32, Todo>>,
}

#[get("/api/todo")]
async fn list_todos(db: web::Data<Database>) -> HttpResponse {
    let db = db.store.lock().unwrap();
    HttpResponse::Ok()
        .json(db.values().collect::<Vec<_>>())
}

#[delete("/api/todo/{id}")]
async fn delete_todo(id: web::Path<u32>, db: web::Data<Database>) -> HttpResponse {
    let mut db = db.store.lock().unwrap();
    match db.remove(&id) {
        Some(_res) => HttpResponse::Ok().body(""),
        None => HttpResponse::BadRequest().body(format!("No todo with id {}", id))
    }
}

#[get("/api/todo/{id}")]
async fn get_todo(id: web::Path<u32>, db: web::Data<Database>) -> HttpResponse {
    let db = db.store.lock().unwrap();
    match db.get(&id) {
        Some(res) => HttpResponse::Ok().json(res),
        None => HttpResponse::NotFound().body(format!("No todo with id {}", id))
    }
}

#[post("/api/todo")]
async fn create_todo(db: web::Data<Database>, body: web::Json<Todo>) -> HttpResponse {
    let mut db = db.store.lock().unwrap();
    let todo_id = body.id;
    db.insert(todo_id, body.into_inner());
    HttpResponse::Created()
        .header(http::header::LOCATION, format!("/api/todo/{}", todo_id))
        .body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match std::env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 8080,
    };

    let db = web::Data::new(Database {
        store: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(|| HttpResponse::Ok()))
            .service(list_todos)
            .service(get_todo)
            .service(create_todo)
            .service(delete_todo)
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .bind((Ipv6Addr::UNSPECIFIED, port))?
    .run()
    .await
}
