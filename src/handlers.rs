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

#[get("/posts/{file_name}")]
async fn blog_post(file_name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("You clicked on {}", &file_name))
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
