mod response_types;

use dotenv::dotenv;
use response_types::{Bar};
use ureq::{Request};
use serde::Deserialize;

pub struct Query {
  pub stock_symbol: String, // Required
  pub timeframe: String, // Required, make Enum
  pub start_time: Option<String>, // Optional
  pub end_time: Option<String>, // Optional
}

impl Query {
    pub fn formatted_string(&self) -> String {
        let mut query_string = format!("?timeframe={}", self.timeframe);

        if let Some(start) = &self.start_time {
            query_string.push_str(&format!("&start={}", start));
        }

        if let Some(end) = &self.end_time {
            query_string.push_str(&format!("&end={}",end));
        }

        query_string
    }
}
pub struct Client;
impl Client {
    const BASE_URL: &str = "https://data.alpaca.markets/v2/stocks";

    fn request(method: &str, address: &str) -> Request {
        dotenv().ok();
        let id_key = std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found");
        let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found");
        ureq::request(method, address)
            .set("APCA-API-KEY-ID", &id_key)
            .set("APCA-API-SECRET-KEY", &secret_key)
    }

    pub fn get_bars(query: Query) -> Vec<Bar> {
        let address = format!("{}/{}/bars{}", Self::BASE_URL, query.stock_symbol, query.formatted_string());

        #[derive(Deserialize)]
        struct Bars {
            bars: Option<Vec<Bar>>,
            symbol: String,
            next_page_token: Option<String>,
        }
        let res: Bars = Self::request("GET", &address)
            .call()
            .expect("Could Not Call API")
            .into_json()
            .expect("Could Not Parse Response Into Json");

        res.bars.expect("No Bars In Response")
    }
}

