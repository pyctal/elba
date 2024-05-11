use chrono::NaiveDateTime;
use diesel::prelude::Insertable;
use diesel::{deserialize::Queryable, sql_types::Bool, Selectable};

use crate::schema::match_data;
use crate::schema::queue_metadata;

#[derive(Queryable, Selectable)]
#[diesel(table_name = match_data)]
pub struct MatchData {
    pub id: String,
    pub start_time: NaiveDateTime,
    pub processed: Bool,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = queue_metadata)]
pub struct QueueMetadata {
    pub hash: String,
    pub task_info: String,
    pub time_inserted: NaiveDateTime,
}
