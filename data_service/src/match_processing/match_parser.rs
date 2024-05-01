use crate::types::{MatchTimelineFrame, PuuidToChampionMapping};
use chrono::{DateTime, TimeDelta};
use riven::models::match_v5::{Match, MatchTimeline};

pub async fn parse_match_timeline(_match_timeline: MatchTimeline) -> crate::types::MatchTimeline {
    // Parse match timeline here.
    let match_frames = _match_timeline.info.frames;

    // println!("Start Time: {:?}", start_time);

    // for ( index, frame) in match_frames.iter().enumerate() {
    //     let frame_events = &frame.events;

    //     println!("Frame: {}, No. of Events: {}", index, frame_events.len());
    //     println!( "Real Match Timestamp Frame {}: {:?}", index, frame.events[0].real_timestamp);

    //     if index == 5 {
    //         break;
    //     }
    // }MatchTimelineFrame

    // let frames_after_convesion = _match_timeline.info.frames
    //     .iter()
    //     .map(|frame| MatchTimelineFrame {
    //         frame_time: TimeDelta::milliseconds(frame.timestamp as i64),
    //         // mapping:
    //         // current_gold: frame.
    //     })
    //     .collect();

    crate::types::MatchTimeline {
        frames: vec![],
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
        let test_match_1: MatchTimeline = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_timeline_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        // Act.
        let response: crate::types::MatchTimeline = parse_match_timeline(test_match_1).await;

        // Assert.
        assert_eq!(response.match_id, "EUW1_6920643858");
    }

    #[tokio::test]
    async fn test_match_timeline_frame_metadata() {
        // Arrange.
        let test_match_1: MatchTimeline = serde_json::from_str(
            fs::read_to_string("src/match_processing/test_data/test_match_timeline_1.txt")
                .unwrap()
                .as_str(),
        )
        .unwrap();

        // Act.
        let response: crate::types::MatchTimeline = parse_match_timeline(test_match_1).await;

        // Assert.
        let frames = response.frames;
        assert!(frames.len() == 42);
        for frame in frames {
            assert!(frame.mappings.len() == 10);
        }
    }
}
