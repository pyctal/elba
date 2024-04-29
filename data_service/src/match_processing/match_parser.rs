use riven::models::match_v5::Match;

use crate::types::PuuidToChampionMapping;

pub async fn parse_match(match_data: Match) -> Vec<PuuidToChampionMapping> {
    // Get match data.

    let to_return : Vec<PuuidToChampionMapping> = match_data
    .info
    .participants.iter().map(|participant| {
        PuuidToChampionMapping {
            puuid: participant.puuid.clone(),
            champion_name: participant.champion_name.clone(),
        }
    }).collect();
    
    // println!("Printing mappings");
    // for mapping in &to_return {
    //     println!("puuid: {}, champion_name: {}", mapping.puuid, mapping.champion_name);
    // }

    to_return
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
