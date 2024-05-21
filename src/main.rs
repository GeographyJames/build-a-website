mod build;
mod handlers;
mod post;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use build::build;
use handlers::{blog_post, index};
use post::{Post, PostError};
use serde::Serialize;

#[derive(Serialize)]
struct AppState {
    posts: Vec<Post>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    let post_results = build();
    let mut errors: Vec<PostError> = Vec::new();
    let mut posts: Vec<Post> = Vec::new();
    for post_result in post_results {
        match post_result {
            Ok(post) => posts.push(post),
            Err(err) => errors.push(err),
        }
    }
    println!("Succesfully converted {:?} post(s) to html", posts.len());
    if !errors.is_empty() {
        println!(
            "failed to convert {:?} post(s) with the following errors:",
            errors.len()
        );
        for error in errors {
            println!("{}", error.message)
        }
    }
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                posts: posts.clone(),
            }))
            .wrap(Logger::default())
            .service(actix_files::Files::new("/static", "static"))
            .service(index)
            .service(blog_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
