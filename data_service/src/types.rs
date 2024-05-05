use chrono::{NaiveDateTime, TimeDelta};

#[derive(PartialEq)]
pub struct ParticipantIdToChampionMapping {
    pub participant_id: String,
    pub champion_name: String,
    pub position: String,
    pub puuid: String,
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
