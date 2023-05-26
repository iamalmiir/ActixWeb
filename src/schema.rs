// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Varchar,
        description -> Nullable<Text>,
        completed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(tasks, users,);
