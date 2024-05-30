mod handlers;
mod post;

use std::str::FromStr;
use std::{os::unix::fs::DirEntryExt, path::PathBuf};

use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::web::ServiceConfig;
use handlers::{blog_post, index, latest_post};
use post::{Post, PostError};
use serde::Serialize;
use shuttle_actix_web::ShuttleActixWeb;
use std::fs::{self, ReadDir};
#[derive(Serialize)]
struct AppState {
    posts: Vec<Post>,
    collections: Vec<String>,
}

fn post_maker(directory_entry: ReadDir, posts: &mut Vec<Result<Post, PostError>>) {
    for directory in directory_entry {
        let dir_res = directory.unwrap();
        let path = dir_res.path();
        if path.is_file() {
            posts.push(Post::from_file_path(&path))
        } else {
            post_maker(fs::read_dir(path.clone()).unwrap(), posts)
        }
    }
}

fn generate_app_state() -> AppState {
    let source_directory = PathBuf::from_str("posts").unwrap();
    let mut collections: Vec<String> = Vec::new();
    let mut errors: Vec<PostError> = Vec::new();
    let mut posts: Vec<Post> = Vec::new();
    let mut post_results: Vec<Result<Post, PostError>> = Vec::new();
    let directory_entry = fs::read_dir(source_directory).unwrap();
    post_maker(directory_entry, &mut post_results);

    for post_result in post_results {
        match post_result {
            Ok(post) => {
                if !collections.contains(&post.collection) {
                    collections.push(post.collection.clone())
                }

                posts.push(post);
            }
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
    posts.sort_by_key(|p| p.date);
    AppState { posts, collections }
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
                .service(Files::new("/static", "./static").show_files_listing())
                .service(latest_post),
        );
    };
    Ok(config.into())
}
