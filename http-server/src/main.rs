use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::fs::{self, File}; // Added fs module for directory operations
use std::io::Write;
use std::path::Path; // For working with file paths
use chrono::Utc; // For generating timestamps

// Define the target directory for saving JSON files
const SAVE_DIR: &str = "json_data";

struct AppState {
    app_name: String,
}

#[get("/service-echo")]
async fn service_echo() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/service")]
async fn service_post() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn manual_echo() -> String {
    // HttpResponse::Ok().body("Hello, world!")
    "Hello world manually.".to_owned()
}

#[post("/echo")]
async fn echo_body(body: web::Bytes) -> impl Responder {
    let body_str = String::from_utf8_lossy(&body);
    println!("Received body: {}", body_str);
    HttpResponse::Ok().body(body) // Echo the body back in the response
}

#[post("/save_json")]
async fn save_json_body(body: web::Bytes) -> impl Responder {
    // 1. Ensure the target directory exists
    if !Path::new(SAVE_DIR).exists() {
        match fs::create_dir_all(SAVE_DIR) {
            Ok(_) => println!("Created directory: {}", SAVE_DIR),
            Err(e) => {
                eprintln!("Error creating directory {}: {}", SAVE_DIR, e);
                return HttpResponse::InternalServerError().body(format!("Failed to create directory: {}", e));
            }
        }
    }

    // 2. Create a unique filename using a timestamp
    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S_%f");
    let filename = format!("received_data_{}.json", timestamp);
    // 3. Construct the full filepath including the directory
    let filepath_buf = Path::new(SAVE_DIR).join(&filename);
    let filepath = filepath_buf.to_str().unwrap_or_default(); // Convert PathBuf to &str for messages

    if filepath.is_empty() {
        eprintln!("Error: Could not construct a valid UTF-8 filepath.");
        return HttpResponse::InternalServerError().body("Failed to construct file path.");
    }


    // 4. Attempt to create and write to the file
    match File::create(&filepath_buf) { // Use PathBuf directly for file creation
        Ok(mut file) => {
            if let Err(e) = file.write_all(&body) {
                eprintln!("Error writing to file {}: {}", filepath, e);
                return HttpResponse::InternalServerError().body(format!("Failed to save data: {}", e));
            }

            println!("Successfully saved body to {}", filepath);

            // Optional: Log the received data (pretty-printed if JSON)
            match serde_json::from_slice::<serde_json::Value>(&body) {
                Ok(json_value) => {
                    match serde_json::to_string_pretty(&json_value) {
                        Ok(pretty_json) => println!("Received JSON data:\n{}", pretty_json),
                        Err(_) => println!("Received data (not valid JSON or failed to pretty print):\n{}", String::from_utf8_lossy(&body)),
                    }
                }
                Err(_) => {
                    println!("Received data (not valid JSON):\n{}", String::from_utf8_lossy(&body));
                }
            }

            HttpResponse::Ok()
                .insert_header(("X-Saved-Filename", filename.clone()))
                .body(body)
        }
        Err(e) => {
            eprintln!("Error creating file {}: {}", filepath, e);
            HttpResponse::InternalServerError().body(format!("Failed to create file: {}", e))
        }
    }
}

#[get("/about")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                //app data pode ser qualquer tipo de dado que vira variável no contexto dos handlers
                app_name: String::from("Actix Web"),
            }))
            .service(service_echo)
            .service(index)
            .service(echo_body)
            .service(save_json_body)
            .route("/", web::get().to(manual_echo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
