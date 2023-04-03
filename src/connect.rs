use dotenv::dotenv;

pub fn connect_to_stock_api() -> serde_json::Value {
  dotenv().ok();
  
  return ureq::get("https://data.alpaca.markets/v2/stocks/SO/bars?timeframe=1Week&start=2023-01-03")
    .set("APCA-API-KEY-ID", &std::env::var("APCA_API_KEY_ID").unwrap())
    .set("APCA-API-SECRET-KEY", &std::env::var("APCA_API_SECRET_KEY").unwrap())
    .call().unwrap()
    .into_json().unwrap();
}