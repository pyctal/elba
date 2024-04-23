use riven::{consts::RegionalRoute, models::match_v5::MatchTimeline, RiotApi};

struct MatchProcessor {
    riot_api: RiotApi,
}

impl MatchProcessor {
    pub async fn get_match(
        &self,
        match_id: &str,
        route: RegionalRoute,
    ) -> Option<riven::models::match_v5::Match> {
        self.riot_api
            .match_v5()
            .get_match(route, match_id)
            .await
            .unwrap()
    }

    pub async fn get_match_timeline(
        &self,
        match_id: &str,
        route: RegionalRoute,
    ) -> Option<MatchTimeline> {
        self.riot_api
            .match_v5()
            .get_timeline(route, match_id)
            .await
            .unwrap()
    }
}
