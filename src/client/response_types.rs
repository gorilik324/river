use serde::Deserialize;

// A Bar is a candle in stock market terms
#[derive(Deserialize, Debug)]
pub struct Bar {
    pub t: String, // Timestamp
    pub o: f32,    // Open
    pub h: f32,    // High
    pub l: f32,    // Low
    pub c: f32,    // Close
    pub v: i32,    // Volume
    pub n: i32,    // Number of trades
    pub vw: f32,   // Volume weighted average
}


#[derive(Deserialize)]
pub struct Trade {
  t: String,
  x: String,
  p: f32,
  s: i32,
  c: [String; 2],
  i: u32,
  z: String
}

