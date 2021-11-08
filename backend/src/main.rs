use actix::Addr;
use actix_files as fs;
use actix_session::CookieSession;
use actix_web::http::{ContentEncoding, StatusCode};
use actix_web::*;

use agent::{agent::AgentManager, resolver::DnsResolver};
use log::info;
use rand::Rng;
use user::auth::Authenticator;

mod agent;
mod user;

const STATIC_DIR: &str = "./static/";
const PAGE_INDEX: &str = "./static/index.html";
const PAGE_NOT_FOUND: &str = "./static/p404.html";

pub struct AppData {
    // session: CookieSession,
    resolver: DnsResolver,
    authenticator: Addr<Authenticator>,
    agents: Addr<AgentManager>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            resolver: DnsResolver::new(),
            authenticator: Authenticator::new(),
            agents: AgentManager::new(),
        }
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self::new()
    }
}

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
            .app_data(AppData::new())
            .wrap(CookieSession::signed(&private_key).secure(false))
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(index)
            .service(user::auth::auth)
            .service(agent::remote::target_validate)
            .service(agent::remote::target_ssh)
            .service(agent::ws::ws_index)
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
