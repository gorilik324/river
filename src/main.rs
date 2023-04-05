use utils::Query;

fn main() {
    let query = Query {
        api_type: String::from("bars"), // Required, make Enum
        stock_symbol: String::from("SO"),
        query_string: String::from("timeframe=1Week&start=2023-01-01")
    };
  let json = query.get_bars();
  println!("{:?}", json);
}
