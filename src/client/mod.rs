mod response_types;

use dotenv::dotenv;
use response_types::Bar;
use serde::Deserialize;
use ureq::Request;

pub struct Query {
    pub stock_symbol: String,       // Required
    pub timeframe: String,          // Required, make Enum
    pub start_time: Option<String>, // Optional
    pub end_time: Option<String>,   // Optional
}
impl Query {
    fn query_string(&self) -> String {
        let mut query_string = format!("timeframe={}", self.timeframe);
        if let Some(start) = &self.start_time {
            query_string.push_str(&format!("&start={}", start));
        }
        if let Some(end) = &self.end_time {
            query_string.push_str(&format!("&end={}", end));
        }
        query_string
    }

    fn build_address_for(&self, api_type: &str) -> String {
        let base_url = "https://data.alpaca.markets/v2/stocks";
        format!(
            "{base_url}/{}/{api_type}?{}",
            self.stock_symbol,
            self.query_string()
        )
    }
}

pub struct Client;
impl Client {
    fn request(method: &str, address: &str) -> Request {
        dotenv().ok();
        let id_key = std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found");
        let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found");
        ureq::request(method, address)
            .set("APCA-API-KEY-ID", &id_key)
            .set("APCA-API-SECRET-KEY", &secret_key)
    }

    pub fn get_bars(q: Query) -> Vec<Bar> {
        #[derive(Deserialize)]
        struct Bars {
            bars: Option<Vec<Bar>>,
            symbol: String,
            next_page_token: Option<String>,
        }
        let address = q.build_address_for("bars");
        let res: Bars = Self::request("GET", &address)
            .call()
            .expect("Could Not Call API")
            .into_json()
            .expect("Could Not Parse Response Into Json");

        res.bars.expect("No Bars In Response")
    }
}
