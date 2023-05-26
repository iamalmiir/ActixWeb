use crate::db_models::Task;
use crate::db_utils::DbActor;
use crate::insertable::NewTask;
use crate::messages::{CreateTask, FetchTasks};
use crate::schema::tasks::dsl::*;
use actix::Handler;
use diesel::{self, prelude::*};
use uuid::Uuid;

impl Handler<FetchTasks> for DbActor {
    type Result = QueryResult<Vec<Task>>;

    fn handle(&mut self, _msg: FetchTasks, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Fetch Tasks: Unable to reach database.");

        tasks.load::<Task>(&mut conn)
    }
}

impl Handler<CreateTask> for DbActor {
    type Result = QueryResult<Task>;

    fn handle(&mut self, _msg: CreateTask, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("Create Task: Unable to reach database.");

        let new_task = NewTask {
            id: Uuid::new_v4(),
            user_id: _msg.user_id,
            title: _msg.title,
            description: _msg.description,
            completed: _msg.completed,
        };

        diesel::insert_into(tasks)
            .values(new_task)
            .returning((
                id,
                user_id,
                title,
                description,
                completed,
                created_at,
                updated_at,
            ))
            .get_result::<Task>(&mut conn)
    }
}
