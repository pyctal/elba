use riven::models::match_v5::Match;

use crate::types::PuuidToChampionMapping;

pub async fn parse_match(match_data: Match) -> Vec<PuuidToChampionMapping> {
    // Get match data.
    vec![]
}

#[cfg(test)]
mod tests {
    use std::fs;

    use riven::models::match_v5::{self, Match};

    use super::parse_match;

    #[tokio::test]
    async fn get_10_mappings_when_parsed() {
        // Arrange.
        let test_match_1: Match = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        // Act.
        let response = parse_match(test_match_1).await;

        // Assert.
        assert_eq!(response.len(), 10);
    }
}
