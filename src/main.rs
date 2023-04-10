use river::{Client, Query, bar::BarSet};

fn main() {
    let q = Query {
        stock_symbol: String::from("BTU"),
        timeframe: String::from("1Week"),
        start_time: Some(String::from("2023-01-01")),
        end_time: None,
    };
    let bars = Client::get_bars(&q.stock_symbol, &q.build_query_string());
    dbg!(BarSet::order_block(&bars));
}
