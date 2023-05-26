use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use uuid::Uuid;

use crate::{
    messages::{CreateTask, CreateUser, FetchTasks, FetchUsers},
    AppState, DbActor,
};
use actix::Addr;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateTaskBody {
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUsers).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to reach database"),
    }
}

#[get("/tasks")]
pub async fn fetch_tasks(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchTasks).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No tasks found"),
        _ => HttpResponse::InternalServerError().json("Unable to reach database"),
    }
}

#[post["/users"]]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db
        .send(CreateUser {
            first_name: body.first_name.to_string(),
            last_name: body.last_name.to_string(),
            username: body.username.to_string(),
            email: body.email.to_string(),
            password: body.password.to_string(),
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}

#[post["/tasks"]]
pub async fn create_task(state: Data<AppState>, body: Json<CreateTaskBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db
        .send(CreateTask {
            user_id: body.user_id,
            title: body.title.to_string(),
            description: body.description.to_string(),
            completed: body.completed,
        })
        .await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create task"),
    }
}
