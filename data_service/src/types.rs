use chrono::{NaiveDateTime, TimeDelta};

#[derive(PartialEq, Clone)]
pub struct ParticipantIdToChampionMapping {
    pub participant_id: String,
    pub champion_name: String,
    pub position: String,
}

pub type ParticipantIdToChampionMappingList = Vec<ParticipantIdToChampionMapping>;

pub trait ParticipantIdToChampionMappingListTrait {
    fn find_champion(&self, participant_id: &str) -> Option<&ParticipantIdToChampionMapping>;
    fn find_opponent(&self, participant_id: &str) -> Option<&ParticipantIdToChampionMapping>;
    fn copy(&self) -> ParticipantIdToChampionMappingList;
}

// TODO: Relocate this implementation to a more appropriate location
impl ParticipantIdToChampionMappingListTrait for ParticipantIdToChampionMappingList {
    fn find_champion(&self, participant_id: &str) -> Option<&ParticipantIdToChampionMapping> {
        self.iter()
            .find(|mapping| mapping.participant_id == participant_id)
    }
    fn find_opponent(&self, participant_id: &str) -> Option<&ParticipantIdToChampionMapping> {
        self.iter().find(|mapping| {
            mapping.participant_id != participant_id
                && self.find_champion(participant_id).unwrap().position == mapping.position
        })
    }
    fn copy(&self) -> ParticipantIdToChampionMappingList {
        self.iter().map(|mapping| mapping.clone()).collect()
    }
}

#[derive(PartialEq)]
pub struct ChampionFrame {
    pub champion_name: String,
    pub opposing_champion_name: String,
    pub position: String,
    pub gold: String,
}

#[derive(PartialEq)]
pub struct MatchTimelineFrame {
    pub mappings: Vec<ChampionFrame>,
    /**
     * Offset of game start time
     */
    pub frame_time: TimeDelta,
}

#[derive(PartialEq)]
pub struct MatchTimeline {
    pub frames: Vec<MatchTimelineFrame>,
    pub match_id: String,
    pub start_time: NaiveDateTime,
}

#[derive(Hash, serde::Serialize, serde::Deserialize)]
pub struct RiotId {
    pub game_name: String,
    pub tag: String,
}
