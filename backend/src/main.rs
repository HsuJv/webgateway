use actix_files as fs;
use actix_session::CookieSession;
use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::*;

use log::info;
use rand::Rng;

mod user;
mod ws;

// pub struct AppState ;

// impl Actor for AppState {
//     type Context = actix::Context<Self>;
// }

const STATIC_DIR: &str = "./static/";
const PAGE_INDEX: &str = "./static/index.html";
const PAGE_NOT_FOUND: &str = "./static/p404.html";

fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || 12)
        .start(log::LevelFilter::Trace)
        .unwrap();
}

#[get("/")]
async fn index(_: HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PAGE_INDEX)?)
}

async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PAGE_NOT_FOUND)?.set_status_code(StatusCode::NOT_FOUND))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    info!("Server starts at http://127.0.0.1:8080");
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    HttpServer::new(move || {
        App::new()
            // .data(AppState)
            .wrap(CookieSession::signed(&private_key).secure(false))
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(index)
            .service(ws::ws_index)
            .service(user::auth::auth)
            .service(
                fs::Files::new("/static", STATIC_DIR)
                    .prefer_utf8(true)
                    .index_file(PAGE_INDEX)
                    .use_etag(true)
                    .default_handler(web::route().to(p404)),
            )
            .default_service(web::route().to(p404))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
