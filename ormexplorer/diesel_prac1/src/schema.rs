diesel::table! {
    table1 (id) {
        id -> Int4,
        name -> Varchar,
        age -> Int4,
        department -> Text,
        working -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(table1,);
