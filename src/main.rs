use actix_identity::IdentityMiddleware;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use chrono::Duration;

mod endpoints;
use endpoints::*;

mod server;

macro_rules! wapp {
    ($e:expr; $($i:ident),+) => {
        $e
            $(
                .service($i)
            )+
    };
}

// How to do Path extractor
// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     let p = format!("<p>Hello {}</p>", name);
//     HttpResponse::Ok().body(p)
// }

// How to do Identity login 
// #[get("/index")]
// async fn index(user: Option<Identity>) -> impl Responder {
//     if let Some(user) = user {
//         format!("Welcome! {}", user.id().unwrap())
//     } else {
//         "Welcome Anonymous!".to_owned()
//     }
// }

//Serve the test VVVVV
// pub const TEST: &str = include_str!(concat!("../src-web/html/", "test", ".html"));
// #[actix_web::get("/test")]
// pub async fn test() -> impl actix_web::Responder{
//     actix_web::HttpResponse::Ok().body(TEST)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move|| {
        wapp!(
            App::new()
            .wrap(IdentityMiddleware::builder()
                .visit_deadline(#[allow(clippy::unwrap_used)] Some(Duration::days(30).to_std().unwrap()))
                .login_deadline(#[allow(clippy::unwrap_used)] Some(Duration::days(365).to_std().unwrap()))
                .build()
            )
            .wrap(
                actix_web::middleware::ErrorHandlers::new()
                .handler(actix_web::http::StatusCode::NOT_FOUND, not_found)
            )
            .service(actix_files::Files::new("/src-web/static", "./src-web/static"));
            homepage, join,
            host, play,
            ws_host, ws_play
        )
    })
    .bind(("127.0.0.1", 8080))?
    // .workers(2)
    .run()
    .await
}

use actix_web::{middleware::ErrorHandlerResponse, dev::ServiceResponse};
fn not_found<B>(res: ServiceResponse<B>) -> actix_web::error::Result<ErrorHandlerResponse<B>> {
    // split service response into request and response components
    let (req, res) = res.into_parts();
  
    // set body of response to modified body
    let res = res.set_body("<center>Page not found!</center>");
  
    // modified bodies need to be boxed and placed in the "right" slot
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();
  
    Ok(ErrorHandlerResponse::Response(res))
}

// use std::sync::Arc;
// use tokio::sync::Mutex;
// use crate::db::Db;
// pub struct AppData {
//     pub db: Arc<Mutex<Db>>,
// }

#[derive(serde::Serialize)]
pub struct EndpointError{
    message: String,
    for_user: bool,
}
impl EndpointError{
    pub fn from_message(message: impl ToString) -> Self{
        let message = message.to_string();
        // println!("{message}");
        Self { message, for_user: false }
    }
    pub fn for_user(mut self) -> Self{
        self.for_user = true;
        self
    }
    pub fn to_js(self) -> HttpResponse{
        HttpResponse::BadRequest().json(self)
    }
    pub fn for_js(message: impl ToString) -> HttpResponse {
        Self::from_message(message).to_js()
    }
    pub fn for_js_user(message: impl ToString) -> HttpResponse {
        Self::from_message(message).for_user().to_js()
    }
    pub fn for_html(message: impl ToString) -> HttpResponse{
        HttpResponse::BadRequest().body(message.to_string())
    }
    // pub fn for_html_stderr() -> HttpResponse{
    //     HttpResponse::BadRequest().body(cmd::sites::ERRHTML)
    // }
}

