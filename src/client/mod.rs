mod query;
pub mod bar;

use dotenv::dotenv;
pub use query::Query;
use serde::Deserialize;
use ureq::Request;
use bar::{Bar};

pub struct Client;
impl Client {
    fn request(method: &str, query: &str) -> Request {
        dotenv().ok();
        let id_key = std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found");
        let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found");
        let base_url = "https://data.alpaca.markets/v2/stocks";

        let address = format!("{base_url}/{query}");
        ureq::request(method, &address)
            .set("APCA-API-KEY-ID", &id_key)
            .set("APCA-API-SECRET-KEY", &secret_key)
    }

    pub fn get_bars(stock_symbol: &str, query_string: &str) -> Vec<Bar> {
        let query = format!("{stock_symbol}/bars?{query_string}");
        #[derive(Deserialize)]
        struct Res {
            bars: Option<Vec<Bar>>,
            symbol: String,
            next_page_token: Option<String>,
        }
        let r: Res = Self::request("GET", &query)
            .call()
            .expect("Could Not Call API")
            .into_json()
            .expect("Could Not Parse Response Into Json");

        r.bars.expect("No Bars In Response")
    }
}
