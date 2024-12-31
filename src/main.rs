use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use diesel_demo::{establish_connection}; //, models::NewPost};
use diesel::prelude::*;
use serde::{Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use diesel_demo::models::Instrument;


mod gui;

#[derive(Serialize)]
struct ApiResponse<T> {
    data: T,
    message: String,
}

fn main() -> std::io::Result<()> {
    // Shared state for the database connection
    let connection = Arc::new(Mutex::new(establish_connection()));

    // Set up a shutdown signal for the GUI
    let (tx, rx) = oneshot::channel();

    // Spawn REST API in a background thread
    let api_connection = Arc::clone(&connection);
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            start_rest_api(api_connection).await.unwrap();
            let _ = tx.send(());
        });
    });

    // Run the GUI on the main thread
    gui::run_gui(connection, rx);

    Ok(())
}

async fn start_rest_api(connection: Arc<Mutex<MysqlConnection>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        let conn = Arc::clone(&connection);
        App::new()
            .app_data(web::Data::new(conn))
            .route("/instruments", web::get().to(get_instruments))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn get_instruments(conn: web::Data<Arc<Mutex<MysqlConnection>>>) -> impl Responder {
    let mut connection = conn.lock().unwrap();

    let results = diesel_demo::schema::instrument::dsl::instrument
        .load::<Instrument>(&mut *connection)
        .expect("Error loading instruments");

    let count = results.len(); // Store the count before moving results
    HttpResponse::Ok().json(ApiResponse {
        data: results, // Move results here
        message: format!("Fetched {} instruments", count), // Use count
    })

}

