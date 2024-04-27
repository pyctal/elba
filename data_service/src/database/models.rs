use diesel::{
    deserialize::Queryable,
    sql_types::{Bool, Timestamp},
    Selectable,
};

use crate::schema::match_data;

#[derive(Queryable, Selectable)]
#[diesel(table_name = match_data)]
pub struct MatchData {
    pub id: String,
    pub start_time: Timestamp,
    pub processed: Bool,
}
