pub struct Query {
    pub stock_symbol: String,       // Required
    pub timeframe: String,          // Required, make Enum
    pub start_time: Option<String>, // Optional
    pub end_time: Option<String>,   // Optional
}
impl Query {
    fn query_string(&self) -> String {
        let mut query_string = format!("timeframe={}", self.timeframe);
        if let Some(start) = &self.start_time {
            query_string.push_str(&format!("&start={}", start));
        }
        if let Some(end) = &self.end_time {
            query_string.push_str(&format!("&end={}", end));
        }
        query_string
    }

    pub fn build_address_for(&self, api_type: &str) -> String {
        let base_url = "https://data.alpaca.markets/v2/stocks";
        format!(
            "{base_url}/{}/{api_type}?{}",
            self.stock_symbol,
            self.query_string()
        )
    }
}
