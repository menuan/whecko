use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use log::{info, error};

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    info!("Responding with \"Hello {}\"", &name);
    format!("Hello {}!", &name)
}

fn start_server() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/hello/{name}", web::get().to(greet))
    })
    .bind("0.0.0.0:8000")?.run()
    // .expect("Can not bind to port 8000")
    // .run()
}

fn main() {
    env_logger::init();
    
    match start_server() {
        Ok(()) =>
            info!("Successfully"),
        Err(err) =>
            error!("Got error {}", err)
    }
}
