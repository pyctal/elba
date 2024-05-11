#[derive(PartialEq)]
pub struct PuuidToChampionMapping {
    pub puuid: String,
    pub champion_name: String,
}

#[derive(Hash, serde::Serialize, serde::Deserialize)]
pub struct RiotId {
    pub game_name: String,
    pub tag: String,
}
