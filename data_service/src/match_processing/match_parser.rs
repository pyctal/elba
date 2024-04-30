use riven::models::match_v5::Match;

use crate::types::PuuidToChampionMapping;

pub async fn parse_match(match_data: Match) -> Vec<PuuidToChampionMapping> {
    match_data
        .info
        .participants
        .iter()
        .map(|participant| PuuidToChampionMapping {
            puuid: participant.puuid.clone(),
            champion_name: participant.champion_name.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use riven::models::match_v5::Match;

    use crate::types::PuuidToChampionMapping;

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

    #[tokio::test]
    async fn check_first_mapping() {
        // Arrange.
        let test_match_1: Match = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let expected_mapping = PuuidToChampionMapping {
            puuid: String::from(
                "9ZjCND00ljx3RZydc1zqxIIvSV_jaDFLeYEMLtvJ2XrUAYhUEVl_Cvvf-qm88PZgMsxbT5yR5ayJog",
            ),
            champion_name: String::from("Aatrox"),
        };

        // Act.
        let response = parse_match(test_match_1).await;

        // Assert.
        assert!(response.contains(&expected_mapping));
    }

    #[tokio::test]
    async fn check_last_mapping() {
        // Arrange.
        let test_match_1: Match = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let expected_mapping = PuuidToChampionMapping {
            puuid: String::from(
                "8ZU_RhZ9awCvMDHbHbtdTM2U_LqQfvUJnu7eYd8w6Umld6D4OeO0PwajuTJSKpCxgALljTpUIn41gA",
            ),
            champion_name: String::from("TahmKench"),
        };

        // Act.
        let response = parse_match(test_match_1).await;

        // Assert.
        assert!(response.contains(&expected_mapping));
    }
}
