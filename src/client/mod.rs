use dotenv::dotenv;
use serde::Deserialize;

pub struct Client {
    base_url: String,
    id_key: (String, String),
    secret_key: (String, String)
}

impl Client {
    pub fn new() -> Self {
        dotenv().ok();
        Self {
            base_url: "https://data.alpaca.markets/v2/stocks".to_string(),
            id_key: (
                "APCA-API-KEY-ID".to_string(),
                std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found"),
            ),
            secret_key: (
                "APCA-API-SECRET-KEY".to_string(),
                std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found"),
            )
        }
    }

    pub fn get_bars_for_stock(&self, stock_symbol: &str, query_string: &str) -> Vec<Bar> {
        #[derive(Deserialize)]
        struct BarsResponse {
            bars: Option<Vec<Bar>>,
            symbol: String,
            next_page_token: Option<String>,
        }
   
        let path = format!("{}/{stock_symbol}/bars?{query_string}", &self.base_url);
        let res: BarsResponse = ureq::get(&path)
            .set(&self.id_key.0, &self.id_key.1)
            .set(&self.secret_key.0, &self.secret_key.1)
            .call()
            .expect("Could Not Call API")
            .into_json()
            .expect("Could Not Parse Response Into Json");
    
        res.bars.expect("No Bars In Response")
    }
}

#[derive(Deserialize, Debug)]
pub struct Bar {
    t: String, // Timestamp
    o: f32,    // Open
    h: f32,    // High
    l: f32,    // Low
    c: f32,    // Close
    v: i32,    // Volume
    n: i32,    // Number of trades
    vw: f32,   // Volume weighted average
}
