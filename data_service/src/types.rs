use chrono::{NaiveDateTime, TimeDelta};

#[derive(PartialEq)]
pub struct PuuidToChampionMapping {
    pub puuid: String,
    pub champion_name: String,
    pub position: String,
}

#[derive(PartialEq)]
pub struct MatchTimelineFrame {
    pub mapping: PuuidToChampionMapping,
    /**
     * Offset of game start time
     */
    pub frame_time: TimeDelta,
    pub current_gold: i32,
}

#[derive(PartialEq)]
pub struct MatchTimeline {
    pub frames: Vec<MatchTimelineFrame>,
    pub match_id: String,
    pub start_time: NaiveDateTime,
}
