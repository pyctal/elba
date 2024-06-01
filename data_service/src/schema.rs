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

diesel::table! {
    champion_matchup_data (champion_name) {
        champion_name -> Varchar,
        opposing_champion_name -> Varchar,
        gold_difference -> Integer,
        game_minute -> Integer,
        day_added -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(match_data, queue_metadata,);
