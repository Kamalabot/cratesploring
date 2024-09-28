diesel::table! {
    practrac (id) {
        id -> Int4,
        sessionname -> Varchar,
        practice -> Int4,
        package -> Varchar,
        completed -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(practrac,);
