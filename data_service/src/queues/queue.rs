use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use chrono::Utc;
use diesel::{PgConnection, RunQueryDsl};

use crate::{database::models::QueueMetadata, types::RiotId};

pub struct Queue {
    pub pg_connection: PgConnection,
}

impl Queue {
    pub fn enqueue_crawl_riot_id_task(&mut self, task: &BeginCrawlRiotIdTask) {
        Self::enqueue(self, task);
    }

    pub fn enqueue_process_match_id_task(&mut self, task: ProcessMatchIdTask) {
        Self::enqueue(self, task);
    }

    pub fn into_inner(self) -> PgConnection {
        self.pg_connection
    }

    fn enqueue<T: Hash + serde::Serialize>(&mut self, task: T) {
        use crate::schema::queue_metadata;

        let new_task = QueueMetadata {
            hash: calculate_hash(&task),
            task_info: serde_json::to_string(&task).unwrap(),
            time_inserted: Utc::now().naive_utc(),
        };

        diesel::insert_into(queue_metadata::table)
            .values(new_task)
            .execute(&mut self.pg_connection)
            .unwrap();
    }
}

fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}

#[derive(Hash, serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
pub struct BeginCrawlRiotIdTask {
    pub riot_id: RiotId,
}

#[derive(Hash, serde::Serialize, serde::Deserialize)]
pub struct ProcessMatchIdTask {
    pub match_id: String,
}
