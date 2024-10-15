#[path= "../handlers.rs"]
mod handlers;
#[path="../routers.rs"]
mod routers;
#[path="../state.rs"]
mod state;

use std::alloc::System;
use std::io;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer};
use state::AppState;
use routers::*;

#[actix_web::rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState{
        health_check_response : "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}