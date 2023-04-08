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

#[derive(Debug)]
pub enum Trend {
    Bullish,
    Bearish,
    Neutral
}

pub struct BarGraph {
    vec: Vec<Bar>
}

impl BarGraph {
    pub fn from(vec: Vec<Bar>) -> BarGraph {
        Self {vec}
    }

    fn all_closes(self) -> Vec<f32> {
        let mut closes = Vec::new();

        for bar in self.vec.iter() {
            closes.push(bar.c)
        }

        return closes;
    }

    pub fn all_opens(self) -> Vec<f32> {
        let mut opens = Vec::new();

        for bar in self.vec.iter() {
            opens.push(bar.o)
        }

        opens
    }

    pub fn signal_and_close(self) -> Vec<(bool, f32)> {
        let mut candle_type = Vec::new();

        for bar in self.vec.iter() {
            let signal = bar.c - bar.o;

            if signal > 0.0 {
                candle_type.push((true, bar.c))
            } else {
                candle_type.push((false, bar.c))
            }
        }

        candle_type
    }

    pub fn macro_trend(self) -> Trend {
        if self.vec.len() < 2 {
            return Trend::Neutral;
        }

        let start = self.vec.first().unwrap();
        let end = self.vec.last().unwrap();

        if start.c > end.c {
            return Trend::Bullish;
        } else {
            return Trend::Bearish;
        }
    }
}
