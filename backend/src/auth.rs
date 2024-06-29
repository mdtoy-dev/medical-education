use crate::db::DbPool;
use crate::models::User;
use crate::schema::users;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

async fn register_user(
    pool: web::Data<DbPool>,
    user_data: web::Json<RegisterData>,
) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    let hashed_password = hash(&user_data.password, DEFAULT_COST).expect("Failed to hash password");

    let new_user = User {
        id: 0,
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        password: hashed_password,
    };

    let result = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn);

    match result {
        Ok(_) => HttpResponse::Ok().json("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn login_user(pool: web::Data<DbPool>, user_data: web::Json<LoginData>) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    let user = users::table
        .filter(users::username.eq(&user_data.username))
        .first::<User>(conn)
        .expect("User not found");

    if verify(&user_data.password, &user.password).unwrap() {
        let my_claims = Claims {
            sub: user.username.clone(),
            exp: 10000000000, // Add appropriate expiration
        };
        let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();

        HttpResponse::Ok().json(token)
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[derive(Deserialize)]
pub struct UpdateProfileData {
    pub email: String,
}

async fn update_profile(
    pool: web::Data<DbPool>,
    user_data: web::Json<UpdateProfileData>,
    req: HttpRequest,
) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .expect("No claims found");

    let result = diesel::update(users::table.filter(users::username.eq(claims.sub)))
        .set(users::email.eq(&user_data.email))
        .execute(conn);

    match result {
        Ok(_) => HttpResponse::Ok().json("Profile updated successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_profile(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");

    let claims = req
        .extensions()
        .get::<Claims>()
        .cloned()
        .expect("No claims found");

    let user = users::table
        .filter(users::username.eq(claims.sub))
        .first::<User>(conn);

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user))
            .route("/profile", web::put().to(update_profile))
            .route("/profile", web::get().to(get_profile)),
    );
}
