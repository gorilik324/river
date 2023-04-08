use river::{Client, Query, Bar};

fn main() {
let bars: Vec<Bar> = vec![
{Bar{
      c: 71.63,
      h: 73.035, // Significant High, BSL - Buy Side Liquidity
      l: 70.12,
      n: 176597,
      o: 71.43,
      t: "2023-01-02T05:00:00Z".to_string(),
      v: 15620581,
      vw: 71.544634
    }},
  {Bar{
      c: 70.32,
      h: 72.195,
      l: 69.33,
      n: 223871,
      o: 71.4,
      t: "2023-01-09T05:00:00Z".to_string(),
      v: 18049668,
      vw: 70.895482
    }},
{Bar{
      c: 67.12,
      h: 70.92,
      l: 65.13,
      n: 173029,
      o: 70.05,
      t: "2023-01-16T05:00:00Z".to_string(),
      v: 15056233,
      vw: 67.83163
    }},
{Bar{
      c: 68.0,
      h: 74.84,
      l: 58.85,
      n: 197442,
      o: 66.95,
      t: "2023-01-23T05:00:00Z".to_string(),
      v: 16882834,
      vw: 67.148639
    }},
{Bar{
      c: 67.27,
      h: 69.44,
      l: 66.24,
      n: 227244,
      o: 67.68,
      t: "2023-01-30T05:00:00Z".to_string(),
      v: 23420636,
      vw: 67.952233
    }},
{Bar{
      c: 66.88,
      h: 67.96,
      l: 65.64,
      n: 165884,
      o: 67.15,
      t: "2023-02-06T05:00:00Z".to_string(),
      v: 14681617,
      vw: 66.848317
    }},
{Bar{
      c: 66.63,
      h: 67.51,
      l: 64.86,
      n: 203826,
      o: 66.8,
      t: "2023-02-13T05:00:00Z".to_string(),
      v: 19481608,
      vw: 66.435289
    }},
{Bar{
      c: 64.56,
      h: 66.27,
      l: 63.71,
      n: 214100,
      o: 66.06,
      t: "2023-02-20T05:00:00Z".to_string(),
      v: 19951204,
      vw: 64.90528
    }},
{Bar{
      c: 64.81,
      h: 65.58,
      l: 61.73, // Significant, SSL - Sell Side Liquidity
      n: 256159,
      o: 64.76,
      t: "2023-02-27T05:00:00Z".to_string(),
      v: 25895510,
      vw: 63.552097
    }},
{Bar{
      c: 63.94,
      h: 65.91,
      l: 63.63,
      n: 233625,
      o: 64.71,
      t: "2023-03-06T05:00:00Z".to_string(),
      v: 19644300,
      vw: 64.678079
    }},
{Bar{
      c: 67.9,
      h: 69.05,
      l: 63.79,
      n: 318809,
      o: 63.79,
      t: "2023-03-13T04:00:00Z".to_string(),
      v: 33855760,
      vw: 67.230145
    }},
{Bar{
      c: 68.59,
      h: 70.42,
      l: 66.06,
      n: 314680,
      o: 68.0,
      t: "2023-03-20T04:00:00Z".to_string(),
      v: 27792653,
      vw: 67.469075
    }},
{Bar{
      c: 69.58,
      h: 69.755,
      l: 68.02,
      n: 244130,
      o: 68.86,
      t: "2023-03-27T04:00:00Z".to_string(),
      v: 23084368,
      vw: 68.92255
    }}
  ];

    fn all_closes(bars: Vec<Bar>) -> Vec<f32> {
        let mut closes = Vec::new();

        for bar in bars.iter() {
            closes.push(bar.c)
        }

        return closes;
    }

    fn all_opens(bars: Vec<Bar>) -> Vec<f32> {
        let mut opens = Vec::new();

        for bar in bars.iter() {
            opens.push(bar.o)
        }

        return opens;
    }

    fn candle_and_close(bars: Vec<Bar>) -> Vec<(bool, f32)> {
        let mut candle_type = Vec::new();

        for bar in bars.iter() {
            let signal = bar.c - bar.o;

            if signal > 0.0 {
                candle_type.push((true, bar.c))
            } else {
                candle_type.push((false, bar.c))
            }
        }

        candle_type
    }

    println!("{:?}", candle_and_close(bars));
//- Order Blocks

//- The last oposite action before a huge move. So if all sells (bear), find the last high (bull) candle. Can be a bearish order block or bullish order block.
//- Bear Order block = Bull action before the bear move
//- Bull Order Block = Bear action before the bull move
//- Rejection Block = Close of either Order block
//- Low of the Bear Order block is significant
//- High of the Bull order block is significant
//- Mean threshold is the 50% of that opposite action either Order Block
//- If an order block gets traded through, or blown through, or broken, or not adhered to, it can become the opposite Order Block.
//- In math terms:
}
