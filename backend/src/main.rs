use actix_files as fs;
use actix_web::http::StatusCode;
use actix_web::*;

const STATIC_DIR: &str = "./static/";
// #[get("/{id}/{name}/index.html")]
// async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
//     format!("Hello {}! id:{}", name, id)
// }

async fn index(_: HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(format!("{}/index.html", STATIC_DIR))?)
}

async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(format!("{}/p404.html", STATIC_DIR))?
        .set_status_code(StatusCode::NOT_FOUND))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                fs::Files::new("/static", STATIC_DIR)
                    .prefer_utf8(true)
                    .index_file(format!("{}/index.html", STATIC_DIR))
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
