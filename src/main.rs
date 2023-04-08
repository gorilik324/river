use river::{Client, Query, bar::BarSet};

fn main() {
    let query = Query {
        stock_symbol: String::from("SO"),
        timeframe: String::from("1Week"),
        start_time: Some(String::from("2023-01-01")),
        end_time: None,
    };
    let bars = Client::get_bars(query);

    dbg!(BarSet::order_block(&bars));
}
