mod query;
mod bar;

use dotenv::dotenv;
pub use query::Query;
use serde::Deserialize;
use ureq::Request;
use bar::{BarGraph, Bar};

pub struct Client;
impl Client {
    fn request(method: &str, address: &str) -> Request {
        dotenv().ok();
        let id_key = std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found");
        let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found");
        ureq::request(method, address)
            .set("APCA-API-KEY-ID", &id_key)
            .set("APCA-API-SECRET-KEY", &secret_key)
    }

    pub fn get_bars(q: Query) -> BarGraph {
        #[derive(Deserialize)]
        struct Res {
            bars: Option<Vec<Bar>>,
            symbol: String,
            next_page_token: Option<String>,
        }
        let address = q.build_address_for("bars");
        let r: Res = Self::request("GET", &address)
            .call()
            .expect("Could Not Call API")
            .into_json()
            .expect("Could Not Parse Response Into Json");

        match r.bars {
            Some(bars) => return BarGraph::from(bars),
            _=> panic!("No Bars In Response")
        }
    }
}
