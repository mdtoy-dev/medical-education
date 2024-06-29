use crate::db::DbPool;
use crate::models::{Course, NewCourse};
use crate::schema::courses;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::AsChangeset;
use serde::Deserialize;

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = courses)]
pub struct CourseData {
    pub title: String,
    pub description: String,
    pub duration: Option<i32>,
    pub category: Option<String>,
}

async fn create_course(
    pool: web::Data<DbPool>,
    course_data: web::Json<CourseData>,
) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    let new_course = NewCourse {
        title: course_data.title.clone(),
        description: course_data.description.clone(),
        duration: course_data.duration,
        category: course_data.category.clone(),
    };

    let result = diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result::<Course>(conn);

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_courses(pool: web::Data<DbPool>) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    let result = courses::table.load::<Course>(conn);

    match result {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_course(
    pool: web::Data<DbPool>,
    course_id: web::Path<i32>,
    course_data: web::Json<CourseData>,
) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    let result = diesel::update(courses::table.find(*course_id))
        .set(&*course_data)
        .get_result::<Course>(conn);

    match result {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_course(pool: web::Data<DbPool>, course_id: web::Path<i32>) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    let result = diesel::delete(courses::table.find(*course_id)).execute(conn);

    match result {
        Ok(_) => HttpResponse::Ok().json("Course deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn init_course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(create_course))
            .route("/", web::get().to(get_courses))
            .route("/{id}", web::put().to(update_course))
            .route("/{id}", web::delete().to(delete_course)),
    );
}
