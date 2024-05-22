mod build;
mod handlers;
mod post;

use std::path::PathBuf;
use std::str::FromStr;

use actix_web::middleware::Logger;
use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpServer};
use build::build;
use handlers::{blog_post, hello_world, index};
use post::{Post, PostError};
use serde::Serialize;
use shuttle_actix_web::ShuttleActixWeb;

#[derive(Serialize)]
struct AppState {
    posts: Vec<Post>,
}
fn generate_app_state() -> AppState {
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
    AppState { posts }
}
//#[actix_web::main]
//async fn main() -> std::io::Result<()> {
//    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
//    let post_results = build();
//    let mut errors: Vec<PostError> = Vec::new();
//    let mut posts: Vec<Post> = Vec::new();
//    for post_result in post_results {
//        match post_result {
//            Ok(post) => posts.push(post),
//            Err(err) => errors.push(err),
//        }
//    }
//    println!("Succesfully converted {:?} post(s) to html", posts.len());
//    if !errors.is_empty() {
//        println!(
//            "failed to convert {:?} post(s) with the following errors:",
//            errors.len()
//        );
//        for error in errors {
//            println!("{}", error.message)
//        }
//    }
//    HttpServer::new(move || {
//        App::new()
//            .app_data(web::Data::new(AppState {
//                posts: posts.clone(),
//            }))
//            .wrap(Logger::default())
//            .service(index)
//            .service(blog_post)
//            .service(actix_files::Files::new("/static", "static"))
//    })
//    .bind(("127.0.0.1", 8080))?
//    .run()
//    .await
//}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_state = web::Data::new(generate_app_state());

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(web::scope("").service(hello_world).app_data(app_state));
    };
    Ok(config.into())
}
