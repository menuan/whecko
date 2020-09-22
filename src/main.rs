use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use log::{info, error};
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn greet(req: HttpRequest) -> impl Responder {
    let mut rng = thread_rng();
    let dice_roll = Uniform::new_inclusive(0, 99).sample(&mut rng);
    if dice_roll > 50 {
        std::process::exit(-1);
    }
    let name = req.match_info().get("name").unwrap_or("World");
    info!("Responding with \"Hello {}\"", &name);
    format!("Hello {}!", &name)
}

fn whoami(req: HttpRequest) -> impl Responder {
    let connection_info = req.connection_info();
    let remote: String = connection_info.remote().unwrap().to_string();
    let vec: Vec<&str> = remote.split(':').collect();
    let ip: &str = vec[0];
    info!("whecko {} - IP: {}", &VERSION, &ip);
    format!("whecko {} - IP: {}", &VERSION, &ip)
}

fn start_server() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/about", web::get().to(whoami))
            .route("/hello/{name}", web::get().to(greet))
    }).bind("0.0.0.0:8000")?.run()
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
