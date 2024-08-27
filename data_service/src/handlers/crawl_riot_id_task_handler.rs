use crate::{
    common::riot_api_layer::RiotApiLayer, queues::queue::BeginCrawlRiotIdTask,
    types::from_custom_regional_route,
};

pub async fn handle_crawl_riot_id_task(task: BeginCrawlRiotIdTask) {
    let riot_id = task.riot_id;
    let api_layer = RiotApiLayer::new().await;
    let _match_ids = api_layer
        .get_matches_by_puuid(&riot_id)
        .await
        .iter()
        .map(|match_id| api_layer.get_match(match_id, from_custom_regional_route(&riot_id.region)));
}
