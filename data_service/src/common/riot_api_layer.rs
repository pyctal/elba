use riven::{
    consts::{Queue, RegionalRoute},
    models::match_v5::MatchTimeline,
    RiotApi,
};

use crate::types::{from_custom_regional_route, RiotId};

use super::get_api_key::get_api_key;

pub struct RiotApiLayer {
    riot_api: RiotApi,
}

impl RiotApiLayer {
    pub async fn new() -> Self {
        Self {
            riot_api: RiotApi::new(get_api_key()),
        }
    }

    pub async fn get_matches_by_puuid(&self, riot_id: &RiotId) -> Vec<String> {
        // Get puuid from riot id.
        let puuid = self
            .riot_api
            .account_v1()
            .get_by_riot_id(RegionalRoute::EUROPE, &riot_id.game_name, &riot_id.tag)
            .await
            .unwrap()
            .expect("Invalid account id");
        self.riot_api
            .match_v5()
            .get_match_ids_by_puuid(
                from_custom_regional_route(&riot_id.region),
                &puuid.puuid,
                None,
                None,
                Some(Queue::SUMMONERS_RIFT_5V5_RANKED_SOLO),
                None,
                None,
                None,
            )
            .await
            .unwrap()
    }

    pub async fn get_match(
        &self,
        match_id: &str,
        regional_route: RegionalRoute,
    ) -> Option<riven::models::match_v5::Match> {
        self.riot_api
            .match_v5()
            .get_match(regional_route, &match_id)
            .await
            .unwrap()
    }

    pub async fn get_match_timeline(
        &self,
        match_id: &str,
        regional_route: RegionalRoute,
    ) -> Option<MatchTimeline> {
        self.riot_api
            .match_v5()
            .get_timeline(regional_route, match_id)
            .await
            .unwrap()
    }
}
