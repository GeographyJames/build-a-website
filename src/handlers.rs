use std::fs;

use crate::AppState;
use actix_web::{get, web, HttpResponse};
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
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(err) => {
            info!("{}", err);
            HttpResponse::InternalServerError().body("something went wrong sad face")
        }
    }
}

#[get("/{file_name}")]
async fn blog_post(file_name: web::Path<String>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("title", &file_name.to_string());
    match fs::read_to_string(format!("_site/posts/{}.html", file_name)) {
        Ok(str) => {
            context.insert("blog_post", &str);
            match TEMPLATE.render("post.html", &context) {
                Ok(rendered) => HttpResponse::Ok().body(rendered),
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
async fn hello_world(state: web::Data<AppState>) -> &'static str {
    let mut context = Context::new();
    context.insert("app_data", &state);
    println!("{:?}", state.posts);
    "Hello wrold"
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn index_should_return_ok_response() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
