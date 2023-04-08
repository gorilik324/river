use river::{Client, Query};

fn main() {
    let query = Query {
        stock_symbol: String::from("SO"),
        timeframe: String::from("1Week"),
        start_time: Some(String::from("2023-01-01")),
        end_time: None,
    };
    let bars = Client::get_bars(query);

    dbg!(bars.macro_trend());
//- Order Blocks

//- The last oposite action before a huge move. So if all sells (bear), find the last high (bull) candle. Can be a bearish order block or bullish order block.
//- Bear Order block = Bull action before the bear move
//- Bull Order Block = Bear action before the bull move
//- Rejection Block = Close of either Order block
//- Low of the Bear Order block is significant
//- High of the Bull order block is significant
//- Mean threshold is the 50% of that opposite action either Order Block
//- If an order block gets traded through, or blown through, or broken, or not adhered to, it can become the opposite Order Block.
//- In math terms:
}
