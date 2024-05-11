// @generated automatically by Diesel CLI.

diesel::table! {
    match_data (id) {
        id -> Varchar,
        start_time -> Timestamp,
        processed -> Bool,
    }
}

diesel::table! {
    queue_metadata (hash) {
        hash -> Varchar,
        task_info -> Varchar,
        time_inserted -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    match_data,
    queue_metadata,
);
