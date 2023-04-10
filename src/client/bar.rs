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
    fn all_highs(bars: &[Bar]) -> Vec<f32> {
        let mut highs = Vec::new();

        for bar in bars.iter() {
            highs.push(bar.h)
        }

        return highs;
    }

    fn all_lows(bars: &[Bar]) -> Vec<f32> {
        let mut lows = Vec::new();

        for bar in bars.iter() {
            lows.push(bar.l)
        }

        return lows;
    }

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

    pub fn bar_trends(bars: &[Bar]) -> Vec<Trend> {
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

    pub fn calc_trend_for_bars(bars: &[Bar]) -> Trend {
      // Wrong signal for BTU
        if bars.len() < 2 {
            return Trend::Neutral;
        }

        let start = bars.first().unwrap();
        let end = bars.last().unwrap();
        if start.c > end.c {
            return Trend::Bearish;
        } else {
            return Trend::Bullish;
        }
    }

    pub fn first_bar_before_streak(bar_trends: &[Trend]) -> usize {
        // Find first element before the start of the streak
        let mut streak_start = 0;
        let mut streak_count = 0;
        let mut max_streak_start = 0;
        let mut max_streak_count = 0;

        for (i, trend) in bar_trends.iter().enumerate() {
            if *trend == Trend::Bearish {
                if streak_count == 0 {
                    streak_start = i;
                }
                streak_count += 1;
            } else {
                if streak_count > max_streak_count {
                    max_streak_count = streak_count;
                    max_streak_start = streak_start;
                }
                streak_count = 0;
            }
        }

        if streak_count > max_streak_count {
            max_streak_count = streak_count;
            max_streak_start = streak_start;
        }

        let start_index = max_streak_start - 1;
        // println!("Biggest consecutive streak: {}", max_streak_count);
        // println!("Starting index of first element before the streak started: {}", start_index);
        start_index
    }

    pub fn order_block(bars: &[Bar]) -> OrderBlock {
        //- The last oposite action before a huge move. So if all sells (bear), find the last high (bull) candle.
        //... can be a bearish order block or bullish order block.
        //- Bear Order block = Bull action before the bear move
        //- Bull Order Block = Bear action before the bull move
        let overall_trend = Self::calc_trend_for_bars(&bars);
        dbg!(&overall_trend);
        let bar_trends = Self::bar_trends(&bars);
        dbg!(&bar_trends);
        let order_block_index = Self::first_bar_before_streak(&bar_trends);
        let bar: &Bar = &bars[order_block_index];
        dbg!(bar);
        let mean_threshold = (bar.h - bar.l) * 0.5;
        //- Rejection Block = Close of either Order block
        //- Low of the Bear Order block is significant
        //- High of the Bull order block is significant
        //- Mean threshold is the 50% of that opposite action either Order Block
        OrderBlock {
            trend: overall_trend,
            close: bar.c,
            high: bar.h,
            low: bar.l,
            mean_threshold
        }
        //- If an order block gets traded through, or blown through, or broken, or not adhered to,
        //... it can become the opposite Order Block.
    }
}
