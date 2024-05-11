use crate::{
    queues::queue::{BeginCrawlRiotIdTask, Queue, SupportedQueueTask},
    types::RiotId,
};

pub mod database;
pub mod match_processing;
pub mod queues;
pub mod schema;
pub mod types;
fn main() {
    let temporary_task = BeginCrawlRiotIdTask {
        riot_id: RiotId {
            game_name: String::from("barun511"),
            tag: String::from("1996"),
        },
    };
    Queue::enqueue_crawl_riot_id_task(temporary_task);
    println!("Hll, world!");
}
