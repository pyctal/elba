diesel::table! {
    match_data (id) {
        id -> VarChar,
        start_time -> Timestamp,
        processed -> Bool,
    }
}
