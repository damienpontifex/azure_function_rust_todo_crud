use actix_web::{HttpRequest, HttpResponse, Result, delete, get, http::header, post, web};
use crate::database::{Database, Todo};

#[get("/api/todo")]
async fn list_todos(db: web::Data<Database>) -> HttpResponse {
    let db = db.store.read().unwrap();
    HttpResponse::Ok()
        .json(db.values().collect::<Vec<_>>())
}

#[delete("/api/todo/{id}")]
async fn delete_todo(id: web::Path<u32>, db: web::Data<Database>) -> HttpResponse {
    let mut db = db.store.write().unwrap();
    match db.remove(&id) {
        Some(_res) => HttpResponse::Ok().finish(),
        None => HttpResponse::BadRequest().body(format!("No todo with id {}", id))
    }
}

#[get("/api/todo/{id}")]
async fn get_todo(id: web::Path<u32>, db: web::Data<Database>) -> HttpResponse {
    let db = db.store.read().unwrap();
    match db.get(&id) {
        Some(res) => HttpResponse::Ok().json(res),
        None => HttpResponse::NotFound().body(format!("No todo with id {}", id))
    }
}

#[post("/api/todo")]
async fn create_todo(req: HttpRequest, db: web::Data<Database>, body: web::Json<Todo>) -> Result<HttpResponse> {
    let mut db = db.store.write().unwrap();
    let todo_id = body.id;
    db.insert(todo_id, body.into_inner());
    let location_url = req.url_for("get_todo", &[todo_id.to_string()])?;
    Ok(HttpResponse::Created()
        .header(header::LOCATION, location_url.as_str())
        .finish())
}