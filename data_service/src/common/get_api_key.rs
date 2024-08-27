use std::env;

use dotenv::dotenv;

pub fn get_api_key() -> String {
    dotenv().ok();
    env::var("RIOT_API_KEY").expect("Riot api key must be set")
}
