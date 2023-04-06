use serde::Deserialize;

// A Bar is a "candle" in stock chart terms.

#[derive(Deserialize, Debug)]
pub struct Bar {
    t: String, // Timestamp
    o: f32,    // Open
    h: f32,    // High
    l: f32,    // Low
    c: f32,    // Close
    v: i32,    // Volume
    n: i32,    // Number of trades
    vw: f32,   // Volume weighted average
}
