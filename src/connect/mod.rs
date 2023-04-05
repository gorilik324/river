mod bar;

use dotenv::dotenv;
use serde::{Deserialize};
use bar::Bar; 

pub struct Query {
    pub stock_symbol: String,
    pub query_string: String
}

// ? Should maybe make the Bars api type an extenstion of query, and maybe even each version of query.
// .. Maybe make query the root of all extensions for the request to the api.
impl Query {
    pub fn get_bars(&self) -> Vec<Bar> {
      dotenv().ok();
    
      let id_key = ("APCA-API-KEY-ID", &std::env::var("APCA_API_KEY_ID").unwrap()); 
      let secret_key = ("APCA-API-SECRET-KEY", &std::env::var("APCA_API_SECRET_KEY").unwrap());
      let base_url = "https://data.alpaca.markets/v2/stocks";
      let Self {stock_symbol, query_string } = &self;
      let path = format!("{base_url}/{stock_symbol}/bars?{query_string}");

      #[derive(Deserialize)]
      struct BarTypeResponse {
          bars: Vec<Bar>,
          symbol: String,
          next_page_token: Option<String>
      }

      //TODO handle edge case of no bars being returned
      let res: BarTypeResponse = ureq::get(&path)
        .set(id_key.0, id_key.1)
        .set(secret_key.0, secret_key.1)
        .call().unwrap()
        .into_json().unwrap();

      return res.bars;
    }
}

