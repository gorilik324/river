use river::{bar::BarSet, indicators::*, Client, Query};
use std::collections::LinkedList;

fn main() {
    let q = Query {
        stock_symbol: String::from("BTU"),
        timeframe: String::from("1Week"),
        start_time: Some(String::from("2023-01-01")),
        end_time: None,
    };
    let bars = Client::get_bars(&q.stock_symbol, &q.build_query_string());
    dbg!(BarSet::order_block(&bars));
    let data = vec![
        25.1, 25.48, 25.34, 25.46, 25.04, 25.54, 24.49, 24.51, 22.54, 22.64, 23.54, 27.01, 27.25,
        26.66, 29.14, 29.68, 29.15, 28.55, 28.82, 28.67, 29.42, 30.19, 30.21, 30.29, 29.78, 30.47,
        28.9, 27.95, 28.3, 28.4, 27.45, 27.01, 25.29, 25.12, 26.485,
    ];
    let test = simple_moving_average(&data);
    println!("{:?}", test);
}
