mod auth;
mod course;
mod db;
mod middleware;
mod models;
mod schema;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix-Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_origin, _req_head| true)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(middleware::Auth)
            .route("/", web::get().to(index))
            .configure(db::init_routes)
            .configure(auth::init_auth_routes)
            .configure(course::init_course_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
