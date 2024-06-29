use crate::models::{Course, NewCourse};
use crate::schema::courses;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("", web::post().to(create_course))
            .route("", web::get().to(get_courses))
            .route("/{id}", web::get().to(get_course))
            .route("/{id}", web::put().to(update_course))
            .route("/{id}", web::delete().to(delete_course)),
    );
}

async fn get_courses(pool: web::Data<DbPool>) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    let result = courses::table.load::<Course>(conn);

    match result {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_course(
    pool: web::Data<DbPool>,
    course_data: web::Json<NewCourse>,
) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    let new_course = diesel::insert_into(courses::table)
        .values(&*course_data)
        .get_result::<Course>(conn);

    match new_course {
        Ok(course) => HttpResponse::Created().json(course),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_course(pool: web::Data<DbPool>, course_id: web::Path<i32>) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    let course = courses::table.find(*course_id).get_result::<Course>(conn);

    match course {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn update_course(
    pool: web::Data<DbPool>,
    course_id: web::Path<i32>,
    course_data: web::Json<Course>,
) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    let updated_course = diesel::update(courses::table.find(*course_id))
        .set(&*course_data)
        .get_result::<Course>(conn);

    match updated_course {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn delete_course(pool: web::Data<DbPool>, course_id: web::Path<i32>) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    let result = diesel::delete(courses::table.find(*course_id)).execute(conn);

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
