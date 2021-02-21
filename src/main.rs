use std::net::{Ipv4Addr, Ipv6Addr};
use actix_web::{App, HttpResponse, HttpServer, middleware, web};

mod services;
mod database;

use crate::database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match std::env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 8080,
    };

    let db = web::Data::new(Database::default());

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(|| HttpResponse::Ok()))
            .service(services::list_todos)
            .service(services::get_todo)
            .service(services::create_todo)
            .service(services::delete_todo)
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .bind((Ipv6Addr::UNSPECIFIED, port))?
    .run()
    .await
}