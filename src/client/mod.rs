mod query;

use dotenv::dotenv;
pub use query::Query;
use serde::Deserialize;
use ureq::Request;

// A Bar is a candle in stock market terms
#[derive(Deserialize, Debug)]
pub struct Bar {
    pub t: String, // Timestamp
    pub o: f32,    // Open
    pub h: f32,    // High
    pub l: f32,    // Low
    pub c: f32,    // Close
    pub v: i32,    // Volume
    pub n: i32,    // Number of trades
    pub vw: f32,   // Volume weighted average
}

#[derive(Deserialize)]
pub struct Trade {
    t: String,
    x: String,
    p: f32,
    s: i32,
    c: [String; 2],
    i: u32,
    z: String,
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
