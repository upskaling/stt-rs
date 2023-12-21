use std::process::Command;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    message: String,
}

async fn api_message(data: web::Json<Message>) -> HttpResponse {
    if data.message != "" {
        Command::new("xdotool")
            .arg("type")
            .arg(data.message.as_str())
            .spawn()
            .expect("failed to execute process")
            .wait()
            .expect("failed to wait for process");
    }

    return HttpResponse::Ok().json(data.into_inner());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = "0.0.0.0";
    log::info!("starting HTTP server at http://{host}:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/api/message").route(web::post().to(api_message)))
            // serve static files
            .service(actix_files::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind((host, 8080))?
    .run()
    .await
}
