use river::get_bars_for_stock;

fn main() {
    let bars = get_bars_for_stock("SO", "timeframe=1Week&start=2023-01-01");
    println!("{:?}", bars);
}
