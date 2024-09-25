// @generated automatically by Diesel CLI.

diesel::table! {
    employee (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
        department -> Text,
        working -> Bool,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    employee,
    posts,
);
