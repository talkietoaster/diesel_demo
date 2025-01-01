use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use diesel_demo::{establish_connection}; //, models::NewPost};
use diesel::prelude::*;
use serde::{Serialize};
use std::sync::{Arc, Mutex};
use std::io::{self, Write};
use std::time::Instant;
use diesel_demo::models::Instrument;
use tokio::sync::oneshot;

mod gui;

#[derive(Serialize)]
struct ApiResponse<T> {
    data: T,
    message: String,
}

enum Mode {
    Rest,
    Cli,
    Gui,
}

fn main() -> std::io::Result<()> {
    let mode = select_mode();
    let connection = Arc::new(Mutex::new(establish_connection()));

    match mode {
        Mode::Rest => {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                start_rest_api(connection).await.unwrap();
            });
        }
        Mode::Cli => {
            start_cli(connection);
        }
        Mode::Gui => {
            let (_shutdown_tx, _shutdown_rx) = oneshot::channel(); // Create the shutdown channel
            gui::run_gui(connection, _shutdown_rx); // Pass both arguments to run_gui
        }
    }

    Ok(())
}


fn select_mode() -> Mode {
    println!("Choose mode:");
    println!("1: REST API");
    println!("2: CLI");
    println!("3: GUI");

    let choice = get_input("Enter your choice: ");
    match choice.trim() {
        "1" => Mode::Rest,
        "2" => Mode::Cli,
        "3" => Mode::Gui,
        _ => {
            println!("Invalid choice. Defaulting to GUI mode.");
            Mode::Gui
        }
    }
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

    HttpResponse::Ok().json(ApiResponse {
        data: results.clone(),
        message: format!("Fetched {} instruments", results.len()),
    })
}

fn start_cli(connection: Arc<Mutex<MysqlConnection>>) {
    let mut conn = connection.lock().unwrap();
    loop {
        println!("\nChoose an action:");
        println!("1: Show instruments");
        println!("2: Exit");

        let choice = get_input("Enter your choice: ");
        match choice.trim() {
            "1" => show_instruments(&mut conn),
            "2" => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn show_instruments(conn: &mut MysqlConnection) {
    use diesel_demo::schema::instrument::dsl::*;
    println!("\nFetching instruments...");
    let start = Instant::now();

    let results = instrument
        .limit(5)
        .load::<Instrument>(conn)
        .expect("Error loading instruments");

    let duration = start.elapsed();
    println!("\nDisplaying {} instruments (fetched in {:.2?})", results.len(), duration);
    for instr in results {
        println!("🎸 {:?}: {:?}\n{:?}\n", instr.id, instr.make, instr.model);
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
