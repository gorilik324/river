use river::{Client, Query};

fn main() {

    let query = Query {
      stock_symbol: String::from("SO"), // Required
      timeframe: String::from("1Week"), // Required, make Enum
      start_time: Some(String::from("2023-01-01")), // Optional
      end_time: None, // Optional
    };
    let bars = Client::get_bars(query);
    println!("{:?}", bars);
}
