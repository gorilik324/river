use river::Client;

fn main() {
    let client = Client::new();

    let bars = client.get_bars_for_stock("SO", "timeframe=1Week&start=2023-01-01");
    println!("{:?}", bars);
}
