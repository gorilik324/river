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

// This is the api response structure for getting bars
#[derive(Deserialize)]
pub struct BarsResponse {
    pub bars: Option<Vec<Bar>>,
    pub symbol: String,
    pub next_page_token: Option<String>,
}
