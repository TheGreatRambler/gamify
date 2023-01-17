use actix_files as fs;
use actix_web::{middleware, App, HttpServer};

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().wrap(middleware::Compress::default()).service(
            fs::Files::new("/", "../client/dist")
                .index_file("index.html")
                .show_files_listing(),
        )
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
