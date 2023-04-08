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

#[derive(Debug, PartialEq, Clone)]
pub enum Trend {
    Bullish,
    Bearish,
    Neutral
}

#[derive(Debug)]
pub struct OrderBlock {
    trend: Trend,
    close: f32,
    high: f32,
    low: f32,
    mean_threshold: f32
}

pub struct BarSet;
impl BarSet {
    fn all_closes(bars: &[Bar]) -> Vec<f32> {
        let mut closes = Vec::new();

        for bar in bars.iter() {
            closes.push(bar.c)
        }

        return closes;
    }

    pub fn all_opens(bars: &[Bar]) -> Vec<f32> {
        let mut opens = Vec::new();

        for bar in bars.iter() {
            opens.push(bar.o)
        }

        opens
    }

    pub fn candle_trends(bars: &[Bar]) -> Vec<Trend> {
        let mut candle_type = Vec::new();

        for bar in bars.iter() {
            let signal = bar.c - bar.o;

            if signal > 0.0 {
                candle_type.push(Trend::Bullish)
            } else {
                candle_type.push(Trend::Bearish)
            }
        }

        candle_type
    }

    pub fn graph_trend(bars: &[Bar]) -> Trend {
        if bars.len() < 2 {
            return Trend::Neutral;
        }

        let start = bars.first().unwrap();
        let end = bars.last().unwrap();

        if start.c > end.c {
            return Trend::Bullish;
        } else {
            return Trend::Bearish;
        }
    }

    pub fn order_block(bars: &[Bar]) -> OrderBlock {
        //- The last oposite action before a huge move. So if all sells (bear), find the last high (bull) candle.
        //... can be a bearish order block or bullish order block.
        //- Bear Order block = Bull action before the bear move
        //- Bull Order Block = Bear action before the bull move

        let trend_of_graph = Self::graph_trend(&bars);

        let bar: &Bar = &bars[0];
        let diff = (bar.o - bar.c) * 0.5;

        OrderBlock {
            trend: trend_of_graph,
            close: bar.c,
            high: bar.h,
            low: bar.l,
            mean_threshold: diff
        }
        //- Rejection Block = Close of either Order block
        //- Low of the Bear Order block is significant
        //- High of the Bull order block is significant
        //- Mean threshold is the 50% of that opposite action either Order Block
        //- If an order block gets traded through, or blown through, or broken, or not adhered to,
        //... it can become the opposite Order Block.
    }
}
