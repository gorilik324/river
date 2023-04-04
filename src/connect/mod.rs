use dotenv::dotenv;

pub struct Query {
    pub api_type: String, // Required, make Enum
    pub stock_symbol: String,
    pub query_string: String
}

// ? Should maybe make the Bars api type an extenstion of query, and maybe even each version of query.
// .. Maybe make query the root of all extensions for the request to the api.
impl Query {
    pub fn send(&self) -> serde_json::Value {
      dotenv().ok();
    
      let id_key = ("APCA-API-KEY-ID", &std::env::var("APCA_API_KEY_ID").unwrap()); 
      let secret_key = ("APCA-API-SECRET-KEY", &std::env::var("APCA_API_SECRET_KEY").unwrap());
      let base_url = "https://data.alpaca.markets/v2/stocks";
      let Self {api_type, stock_symbol, query_string } = &self;
      let path = format!("{base_url}/{stock_symbol}/{api_type}?{query_string}");

      return ureq::get(&path)
        .set(id_key.0, id_key.1)
        .set(secret_key.0, secret_key.1)
        .call().unwrap()
        .into_json().unwrap();
    }
}

