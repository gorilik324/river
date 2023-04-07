mod response_types;

use dotenv::dotenv;
use response_types::{Bar, BarsResponse};
use ureq::{Request};

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

    pub fn get_bars_for_stock(stock_symbol: &str, query_string: &str) -> Vec<Bar> {
        let address = format!("{}/{stock_symbol}/bars?{query_string}", Self::BASE_URL);
        let res: BarsResponse = Self::request("GET", &address)
            .call()
            .expect("Could Not Call API")
            .into_json()
            .expect("Could Not Parse Response Into Json");
    
        res.bars.expect("No Bars In Response")
    }
}

