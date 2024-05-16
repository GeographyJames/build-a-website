use actix_web::{get, HttpResponse};
use lazy_static::lazy_static;
use log::info;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATE: Tera = Tera::new("templates/**/*.html").unwrap();
}

#[get("/")]
async fn index() -> HttpResponse {
    match TEMPLATE.render("index.html", &Context::new()) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(err) => {
            info!("{}", err);
            HttpResponse::InternalServerError().body("something went wrong sad face")
        }
    }
}

#[get("/my_first_post")]
async fn blog_post() -> HttpResponse {
    let mut context = Context::new();
    context.insert("title", "My first  post");
    context.insert("content", "Here is some interesting blog content!");
    match TEMPLATE.render("post.html", &context) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(err) => {
            info!("{}", err);
            HttpResponse::InternalServerError().body("oh dear it didnt work")
        }
    }
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
