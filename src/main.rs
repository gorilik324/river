use utils::Query;

fn main() {
    let query = Query {
        stock_symbol: String::from("SO"),
        query_string: String::from("timeframe=1Week&start=2023-01-01")
    };
  let json = query.get_bars();
  println!("{:?}", json[0]);
}
