use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use chrono::Utc;
use diesel::RunQueryDsl;

use crate::{
    database::{database_client::establish_connection, models::QueueMetadata},
    types::RiotId,
};

pub struct Queue {}

impl Queue {
    pub fn enqueue_crawl_riot_id_task(task: BeginCrawlRiotIdTask) {
        Self::enqueue(task);
    }

    pub fn enqueue_process_match_id_task(task: ProcessMatchIdTask) {
        Self::enqueue(task);
    }

    fn enqueue<T: Hash + serde::Serialize>(task: T) {
        use crate::schema::queue_metadata;

        let connection = &mut establish_connection();

        let new_task = QueueMetadata {
            hash: calculate_hash(&task),
            task_info: serde_json::to_string(&task).unwrap(),
            time_inserted: Utc::now().naive_utc(),
        };

        diesel::insert_into(queue_metadata::table)
            .values(new_task)
            .execute(connection)
            .unwrap();
    }
}

fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}

#[derive(Hash, serde::Serialize, serde::Deserialize)]
pub struct BeginCrawlRiotIdTask {
    pub riot_id: RiotId,
}

#[derive(Hash, serde::Serialize, serde::Deserialize)]
pub struct ProcessMatchIdTask {
    pub match_id: String,
}
