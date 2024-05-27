use std::fs;

use crate::AppState;
use actix_files::NamedFile;
use actix_web::{get, web, HttpResponse, Responder};
use comrak::{markdown_to_html, Options};
use lazy_static::lazy_static;
use log::info;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATE: Tera = Tera::new("templates/**/*.html").unwrap();
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("app_data", &data);
    match TEMPLATE.render("index.html", &context) {
        Ok(rendered) => {
            info!("lodaing template");
            HttpResponse::Ok().body(rendered)
        }
        Err(err) => {
            info!("{}", err);
            HttpResponse::InternalServerError().body("something went wrong sad face")
        }
    }
}

#[get("/{post_name}")]
async fn blog_post(post_name: web::Path<String>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("title", &post_name.to_string());

    match fs::read_to_string(format!("posts/{}.md", post_name)) {
        Ok(str) => {
            context.insert("blog_post", &markdown_to_html(&str, &Options::default()));
            match TEMPLATE.render("post.html", &context) {
                Ok(rendered) => {
                    info!("loading post");
                    HttpResponse::Ok().body(rendered)
                }
                Err(err) => {
                    info!("{}", err);
                    HttpResponse::InternalServerError().body("something went wrong sad face")
                }
            }
        }
        Err(err) => {
            info!("{}", err);
            HttpResponse::InternalServerError().body("something went wrong sad face")
        }
    }
}

#[get("/")]
async fn hello_world(state: web::Data<AppState>) -> impl Responder {
    NamedFile::open_async("static/html/test.html").await
}
