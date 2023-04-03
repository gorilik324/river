use util::connect_to_stock_api;

fn main() {
  let json = connect_to_stock_api();
  println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
