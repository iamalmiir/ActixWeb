use crate::schema::{tasks, users};
use diesel::Insertable;
use serde::Serialize;
use uuid::Uuid;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=tasks)]
pub struct NewTask {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
}
