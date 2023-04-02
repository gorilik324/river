extern crate dotenv;

use dotenv::dotenv;

fn main() {
  dotenv().ok();
  
  let json: serde_json::Value = ureq::get("https://data.alpaca.markets/v2/stocks/SO/bars?timeframe=1Week&start=2023-01-03")
    .set("APCA-API-KEY-ID", &std::env::var("APCA_API_KEY_ID").unwrap())
    .set("APCA-API-SECRET-KEY", &std::env::var("APCA_API_SECRET_KEY").unwrap())
    .call().unwrap()
    .into_json().unwrap();

  println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
