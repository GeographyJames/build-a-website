mod handlers;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use handlers::{blog_post, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(actix_files::Files::new("/static", "static"))
            .service(index)
            .service(blog_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
