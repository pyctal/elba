use data_service::{
    database::models::QueueMetadata,
    queues::queue::{BeginCrawlRiotIdTask, Queue},
    schema::queue_metadata::dsl::*,
    types::{to_custom_regional_route, RiotId},
};
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper};
use riven::consts::RegionalRoute;

use crate::common::get_test_connection;

#[test]
fn test_queue_insertion() {
    // Arrange.
    let queue_connection = get_test_connection();
    let mut queue = Queue {
        pg_connection: queue_connection,
    };
    let example_riot_id_task = BeginCrawlRiotIdTask {
        riot_id: RiotId {
            game_name: String::from("ExampleGame"),
            tag: String::from("ExampleTag"),
            region: to_custom_regional_route(&RegionalRoute::EUROPE),
        },
    };

    // Act.
    queue.enqueue_crawl_riot_id_task(&example_riot_id_task);

    // Assert.
    let mut returned_connection = queue.into_inner();
    let inserted_data = queue_metadata
        .select(QueueMetadata::as_select())
        .load(&mut returned_connection)
        .unwrap();
    assert_eq!(inserted_data.len(), 2);
    let actual_riot_id_task: BeginCrawlRiotIdTask =
        serde_json::from_str::<BeginCrawlRiotIdTask>(&inserted_data[0].task_info.as_str()).unwrap();
    assert_eq!(example_riot_id_task, actual_riot_id_task);
}
