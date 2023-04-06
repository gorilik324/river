mod response_types;

use dotenv::dotenv;
use response_types::Bar;
use serde::Deserialize;

pub fn get_bars_for_stock(stock_symbol: &str, query_string: &str) -> Vec<Bar> {
    dotenv().ok();

    let base_url = "https://data.alpaca.markets/v2/stocks";
    let id_key = (
        "APCA-API-KEY-ID",
        &std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found"),
    );
    let secret_key = (
        "APCA-API-SECRET-KEY",
        &std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found"),
    );

    #[derive(Deserialize)]
    struct ApiResponse {
        bars: Option<Vec<Bar>>,
        symbol: String,
        next_page_token: Option<String>,
    }

    let path = format!("{base_url}/{stock_symbol}/bars?{query_string}");
    let res: ApiResponse = ureq::get(&path)
        .set(id_key.0, id_key.1)
        .set(secret_key.0, secret_key.1)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");

    res.bars.expect("No Bars In Response")
}
