// @generated automatically by Diesel CLI.

diesel::table! {
    news (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        published -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    secrets (id) {
        id -> Int4,
        userid -> Int4,
        title -> Varchar,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    news,
    secrets,
    users,
);