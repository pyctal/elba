use crate::types::{ChampionFrame, MatchTimelineFrame, PuuidToChampionMapping};
use chrono::{DateTime, TimeDelta};
use riven::models::match_v5::{
    Match, MatchTimeline, MatchTimelineInfoFrameParticipantFrame, Participant,
};
use std::collections::HashMap;

pub async fn parse_match_timeline(
    _match_timeline: MatchTimeline,
    _match: Match,
) -> crate::types::MatchTimeline {
    // Parse match timeline here.
    let match_frames = _match_timeline.info.frames;

    let participant_id_object_map: HashMap<i32, Participant> = _match
        .info
        .participants
        .iter()
        .map(|p| (p.participant_id, p.clone()))
        .collect();

    let participant_id_opponent_id_map: HashMap<i32, i32> = _match
        .info
        .participants
        .iter()
        .map(|p| {
            (
                p.participant_id,
                get_opponent_player_id(p.participant_id, _match.clone()),
            )
        })
        .collect();

    let mapped_frames = match_frames
        .iter()
        .map(|frame| {
            let participants_frame_as_hashmap: HashMap<
                String,
                MatchTimelineInfoFrameParticipantFrame,
            > = serde_json::from_value(
                serde_json::to_value(frame.participant_frames.clone()).unwrap(),
            )
            .unwrap();

            let champion_mappings: Vec<ChampionFrame> = participants_frame_as_hashmap
                .iter()
                .map(|(player_id, player)| {
                    let current_player = participant_id_object_map
                        .get(&player_id.parse::<i32>().unwrap())
                        .unwrap();
                    let opposing_player = participant_id_object_map
                        .get(
                            &participant_id_opponent_id_map
                                .get(&player_id.parse::<i32>().unwrap())
                                .unwrap(),
                        )
                        .unwrap();

                    ChampionFrame {
                        champion_name: current_player.champion_name.clone(),
                        opposing_champion_name: opposing_player.champion_name.clone(),
                        position: calculate_position(current_player.individual_position.as_str()),
                        gold: player.current_gold.to_string(),
                    }
                })
                .collect();

            for champion in &champion_mappings {
                println!(
                    "Champ: {} vs: {} - Position: {} Gold {}",
                    champion.champion_name,
                    champion.opposing_champion_name,
                    champion.position,
                    champion.gold,
                );
            }

            MatchTimelineFrame {
                mappings: champion_mappings,
                frame_time: TimeDelta::seconds(frame.timestamp as i64),
            }
        })
        .collect();

    crate::types::MatchTimeline {
        frames: mapped_frames,
        match_id: _match_timeline.metadata.match_id.clone(),
        // match_id: String::from(""),
        start_time: DateTime::from_timestamp_millis(
            match_frames[0].events[0].real_timestamp.unwrap(),
        )
        .expect("invalid timestamp")
        .naive_utc(),
    }
}

pub async fn get_puuid_to_champion_mapping(match_data: Match) -> Vec<PuuidToChampionMapping> {
    match_data
        .info
        .participants
        .iter()
        .map(|participant| PuuidToChampionMapping {
            puuid: participant.puuid.clone(),
            champion_name: participant.champion_name.clone(),
            position: calculate_position(participant.individual_position.as_str()),
        })
        .collect()
}

fn get_opponent_player_id(current_player_id: i32, match_data: Match) -> i32 {
    let current_player_position = calculate_position(
        &match_data
            .info
            .participants
            .iter()
            .find(|p| p.participant_id == current_player_id)
            .unwrap()
            .individual_position,
    );

    match_data
        .info
        .participants
        .iter()
        .find(|p| {
            p.participant_id != current_player_id
                && calculate_position(p.individual_position.as_str()) == current_player_position
        })
        .unwrap()
        .participant_id
}

fn calculate_position(individual_position: &str) -> String {
    match individual_position {
        "UTILITY" => String::from("SUPPORT"),
        other => String::from(other),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use riven::models::match_v5::{Match, MatchTimeline};

    use crate::{
        match_processing::match_parser::parse_match_timeline, types::PuuidToChampionMapping,
    };

    use super::get_puuid_to_champion_mapping;

    #[tokio::test]
    async fn test_puuid_to_champion_mapping_get_10_mappings_when_parsed() {
        // Arrange.
        let test_match_1: Match = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        // Act.
        let response = get_puuid_to_champion_mapping(test_match_1).await;

        // Assert.
        assert_eq!(response.len(), 10);
    }

    #[tokio::test]
    async fn test_puuid_to_champion_mapping_check_first_mapping() {
        // Arrange.
        let test_match_1: Match = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let expected_mapping = PuuidToChampionMapping {
            puuid: String::from(
                "qqtv94VdR_eGjsWvHWveZ4H9erzHsYh-xtJ8adL9CSvELZUakXN7JFZ2JUK7gmZoXB06dT0eiyFJ4Q",
            ),
            champion_name: String::from("Aatrox"),
            position: String::from("TOP"),
        };

        // Act.
        let response = get_puuid_to_champion_mapping(test_match_1).await;

        // Assert.
        assert!(response.contains(&expected_mapping));
    }

    #[tokio::test]
    async fn test_puuid_to_champion_mapping_check_last_mapping() {
        // Arrange.
        let test_match_1: Match = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let expected_mapping = PuuidToChampionMapping {
            puuid: String::from(
                "0h2hQQduHGMct9KBtIBqqPFez_Qva73HXPiSl5vaMGUVWcEJO_e2jMBRS6ZJhMCevJUQ8RWd-gy55Q",
            ),
            champion_name: String::from("TahmKench"),
            position: String::from("SUPPORT"),
        };

        // Act.
        let response = get_puuid_to_champion_mapping(test_match_1).await;

        // Assert.
        assert!(response.contains(&expected_mapping));
    }

    #[tokio::test]
    async fn test_match_timeline_can_parse_match_id() {
        // Arrange.
        let test_match_1: Match = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        let test_match_timeline_1: MatchTimeline = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_timeline_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        // Act.
        let response: crate::types::MatchTimeline =
            parse_match_timeline(test_match_timeline_1, test_match_1).await;

        // Assert.
        assert_eq!(response.match_id, "EUW1_6920643858");
    }

    #[tokio::test]
    async fn test_match_timeline_frame_metadata() {
        // Arrange.
        let test_match_1: Match = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        let test_match_timeline_1: MatchTimeline = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_timeline_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();
        // Act.
        let response: crate::types::MatchTimeline =
            parse_match_timeline(test_match_timeline_1, test_match_1).await;

        // Assert.
        let frames = response.frames;
        assert!(frames.len() == 42);
        for frame in frames {
            assert!(frame.mappings.len() == 10);
        }
    }

    #[tokio::test]
    async fn test_match_timeline_first_frame_data() {
        // Arrange.
        let test_match_1: Match = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        let test_match_timeline_1: MatchTimeline = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_timeline_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        // Act.
        let response: crate::types::MatchTimeline =
            parse_match_timeline(test_match_timeline_1, test_match_1).await;

        // Assert.
        let frames = response.frames;
        assert!(frames.len() == 42);
        for mapping in &frames[0].mappings {
            assert_eq!(mapping.gold.parse::<i32>().unwrap(), 500);
        }
    }
}
