// @generated automatically by Diesel CLI.

diesel::table! {
    courses (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        duration -> Nullable<Int4>,
        category -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    users,
);
