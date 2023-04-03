use dotenv::dotenv;

struct Query {
  base_url: String, // This is static
  api_type: String, // Required, make Enum
  stock_symbol: String, // Required
  timeframe: String, // Required, make Enum
  start: Option<String>, // Optional
  end: Option<String>, // Optional
  limit: u32, // Optional
  page_token: Option<String>, // Optional
  asof: Option<String>, // Optional
  adjustment: Option<String>, // Optional
  feed: Option<String> // Optional, default is free version
}

// ? Should maybe make the Bars api type an extenstion of query, and maybe even each version of query.
// .. Maybe make query the root of all extensions for the request to the api.
impl Query {
  fn new(stock_symbol: String, api_type: String ) -> Self {
    Query {
      base_url: String::from("https://data.alpaca.markets/v2/stocks"),
      api_type,
      stock_symbol,
    }
  }
}

pub fn send_query() -> serde_json::Value {
  dotenv().ok();
  
  return ureq::get("https://data.alpaca.markets/v2/stocks/SO/bars?timeframe=1Week&start=2023-01-03")
    .set("APCA-API-KEY-ID", &std::env::var("APCA_API_KEY_ID").unwrap())
    .set("APCA-API-SECRET-KEY", &std::env::var("APCA_API_SECRET_KEY").unwrap())
    .call().unwrap()
    .into_json().unwrap();
}