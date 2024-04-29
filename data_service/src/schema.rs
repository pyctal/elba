// @generated automatically by Diesel CLI.

diesel::table! {
    match_data (id) {
        id -> Varchar,
        start_time -> Timestamp,
        processed -> Bool,
    }
}
