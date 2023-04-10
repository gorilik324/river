// Build the queries that are passed to the api through
// ... the query string params.j

pub struct Query {
    pub stock_symbol: String,
    pub timeframe: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}
impl Query {
    pub fn build_query_string(&self) -> String {
        let mut args = format!("timeframe={}", self.timeframe);
        if let Some(start) = &self.start_time {
            args.push_str(&format!("&start={}", start));
        }
        if let Some(end) = &self.end_time {
            args.push_str(&format!("&end={}", end));
        }
        args
    }
}
