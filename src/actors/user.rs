use crate::db_models::User;
use crate::utils::DbActor;
use crate::insertable::NewUser;
use crate::messages::{CreateUser, FetchUsers};
use crate::schema::users::dsl::*;
use actix::Handler;
use diesel::{self, prelude::*};
use uuid::Uuid;

impl Handler<FetchUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: FetchUsers, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch Users: Unable to reach database.");

        users.load::<User>(&mut conn)
    }
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, _msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Create User: Unable to reach database.");

        let new_user = NewUser {
            id: Uuid::new_v4(),
            first_name: _msg.first_name,
            last_name: _msg.last_name,
            username: _msg.username,
            email: _msg.email,
            password: _msg.password,
        };

        diesel::insert_into(users)
            .values(new_user)
            .returning((
                id, first_name, last_name, username, email, password, created_at, updated_at,
            ))
            .get_result::<User>(&mut conn)
    }
}
