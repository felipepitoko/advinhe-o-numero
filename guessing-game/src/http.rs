use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// This function will be called for requests to the "/" path
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main] // Actix Web uses async/await, so we need this attribute
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello)) // Map the "hello" function to the "/" path and GET method
    })
    .bind("127.0.0.1:8080")? // Bind the server to listen on this address and port
    .run() // Start the server
    .await
}