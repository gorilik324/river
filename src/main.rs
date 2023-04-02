extern crate sail;

use sail::{Request, Response};
  
fn main() {
  let req = Request {
    method: String::from("GET"),
    resource_path: String::from("/products"),
    query_params: String::new(),
    host: String::from("www.dummyjson.com"),
    port: 443,
  };
 
  let res: Response = sail::send(req);

  println!("{}", res.body);

}
