use utils::Query;

fn main() {
    let query = Query {
        api_type: String::from("bars"), // Required, make Enum
        stock_symbol: String::from("SO"),
        query_string: String::from("timeframe=1Week&start=2023-01-01")
    };
  let json = query.send();
  println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
