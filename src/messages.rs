use crate::models::{Task, User};
use actix::Message;
use diesel::QueryResult;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUsers;

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Task>>")]
pub struct FetchTasks;

#[derive(Message)]
#[rtype(result = "QueryResult<Task>")]
pub struct CreateTask {
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
