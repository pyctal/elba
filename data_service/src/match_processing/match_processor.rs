use riven::{consts::RegionalRoute, models::match_v5::MatchTimeline, RiotApi};

pub struct MatchProcessor {
    riot_api: RiotApi,
}

impl MatchProcessor {
    pub async fn new(api_key: String) -> Self {
        Self {
            riot_api: RiotApi::new(api_key),
        }
    }

    pub async fn get_match(
        &self,
        match_id: &str,
        regional_route: RegionalRoute,
    ) -> Option<riven::models::match_v5::Match> {
        self.riot_api
            .match_v5()
            .get_match(regional_route, match_id)
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
