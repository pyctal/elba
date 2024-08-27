// @generated automatically by Diesel CLI.

diesel::table! {
    champion_matchup_data (champion_name) {
        champion_name -> Varchar,
        opposing_champion_name -> Varchar,
        gold_difference -> Int4,
        game_minute -> Int4,
        day_added -> Timestamp,
    }
}

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
    champion_matchup_data,
    match_data,
    queue_metadata,
);
