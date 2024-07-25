use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(hello))
        .bind("localhost:5001") // Specify the address and port here
        .expect("Cannot bind to localhost:5001"); // Handle the error

    server.run().await
}
