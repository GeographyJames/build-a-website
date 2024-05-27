mod handlers;
mod post;

use std::path::PathBuf;
use std::str::FromStr;

use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::web::ServiceConfig;
use actix_web::{web, App, HttpServer};
use handlers::{blog_post, hello_world, index};
use post::{Post, PostError};
use serde::Serialize;
use shuttle_actix_web::ShuttleActixWeb;
use std::fs;

#[derive(Serialize)]
struct AppState {
    posts: Vec<Post>,
}
fn generate_app_state() -> AppState {
    let source_directory = PathBuf::from_str("posts").unwrap();
    let target_directory = PathBuf::from_str("_site/posts").unwrap();
    let mut errors: Vec<PostError> = Vec::new();
    let mut posts: Vec<Post> = Vec::new();
    let mut post_results: Vec<Result<Post, PostError>> = Vec::new();
    let directory_entry = fs::read_dir(source_directory).unwrap();
    for directory in directory_entry {
        let post_result = Post::from_file_path(&(directory.unwrap().path()));
        post_results.push(post_result.clone());
        if let Ok(post) = post_result {
            let mut target_filename = target_directory.clone();
            target_filename.push(format!("{}.html", &post.file_name));
        }
    }
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

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_state = web::Data::new(generate_app_state());

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .app_data(app_state)
                .wrap(Logger::default())
                .service(index)
                .service(blog_post)
                .service(Files::new("/static", "./static").show_files_listing()),
        );
    };
    Ok(config.into())
}
