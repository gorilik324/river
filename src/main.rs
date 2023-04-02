extern crate sail;
extern crate serde_json;

use sail::{Request, Response};
use serde_json::{Result, Value, json};

fn main() {
  let req = Request {
    method: String::from("GET"),
    resource_path: String::from("/products"),
    query_params: String::new(),
    host: String::from("www.dummyjson.com"),
    port: 443,
  };
 
  let res: Response = sail::send(req);

  let v: Value = serde_json::from_str(&res.body).unwrap();

  println!("{}", serde_json::to_string_pretty(&v).unwrap());
}
