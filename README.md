# River

## Part I

#### Timeframe

- 60 Days to Present, weekly bars
- Determine where price is being drawn to next

  - Where is the nearest liquidity pool or imbalance price will be magnitized to. Price is being drawn a certain way
  - Liquidity pool is a pool of orders, aka stop losses, a point at which in the chart has significant hight or low that stops are placed under
    - A significant high would be somewhere where price tops out and makes a new low
      - Price tops out means that it goes down from there. The highest price that it's reached to for that price leg
      - Price leg is a stream of orders, price goes from. RTO, BOS.
    - A significant low would be where price bottoms out and makes a new high
  - Calculating liquidity - where is price chasing

    - Where are are the significant highs and lows around me, set the range, find the values
    - That determines your range
    - Inside that range, going to be looking for reference points that are called PD arrays. Order blocks, fvgs, breakers, and other imbalances.
    - You need the PD arrays that are untapped or breached yet.
    - PD arrays (true support resistance) are specific areas in price that is going to be needed to return to or launched from.

      - Order Blocks

        - The last oposite action before a huge move. So if all sells (bear), find the last high (bull) candle. Can be a bearish order block or bullish order block.
        - Bear Order block = Bull action before the bear move
        - Bull Order Block = Bear action before the bull move
        - Rejection Block = Close of either Order block
        - Mean threshold is the 50% of that opposite action either Order Block
        - If an order block gets traded through, or blown through, or broken, or not adhered to, it can become the opposite Order Block.

      - Order Blocks II

        - Find all order blocks
        - ! Fix the macro trend problem
        - Low of the Bull Order block is significant
        - High of the Bear order block is significant
        - Find Highs and lows of the spread
        - The most significant blocks are the ones that are closer to the most recent price, usually the most recent block.
        - The most significant ones are closer to price in general.
        - Find an order block on the upper side and lower side of the spectrum. A bearish and bullish block.
        - An order block's low/high also NEEDS to be broken by the next series of blocks.
        - Identify the high of the session of and low of the session, weekly.
        - As long as price moves away, its a block.

      - Fair Value Gap
      - Breakers
      - Volume Imbalances
      - Mean Threshold = 50% of an order block
      - Consequent incrochment = 50% of wick
      - New week opening gap = gap between last fridays closing candle, and sunday's opening candle

#### Example

```json
{
  "bars": [
    {
      "c": 71.63,
      "h": 73.035, // Significant High, BSL - Buy Side Liquidity
      "l": 70.12,
      "n": 176597,
      "o": 71.43,
      "t": "2023-01-02T05:00:00Z",
      "v": 15620581,
      "vw": 71.544634
    },
    {
      "c": 70.32,
      "h": 72.195,
      "l": 69.33,
      "n": 223871,
      "o": 71.4,
      "t": "2023-01-09T05:00:00Z",
      "v": 18049668,
      "vw": 70.895482
    },
    {
      "c": 67.12,
      "h": 70.92,
      "l": 65.13,
      "n": 173029,
      "o": 70.05,
      "t": "2023-01-16T05:00:00Z",
      "v": 15056233,
      "vw": 67.83163
    },
    {
      "c": 68,
      "h": 74.84,
      "l": 58.85,
      "n": 197442,
      "o": 66.95,
      "t": "2023-01-23T05:00:00Z",
      "v": 16882834,
      "vw": 67.148639
    },
    {
      "c": 67.27,
      "h": 69.44,
      "l": 66.24,
      "n": 227244,
      "o": 67.68,
      "t": "2023-01-30T05:00:00Z",
      "v": 23420636,
      "vw": 67.952233
    },
    {
      "c": 66.88,
      "h": 67.96,
      "l": 65.64,
      "n": 165884,
      "o": 67.15,
      "t": "2023-02-06T05:00:00Z",
      "v": 14681617,
      "vw": 66.848317
    },
    {
      "c": 66.63,
      "h": 67.51,
      "l": 64.86,
      "n": 203826,
      "o": 66.8,
      "t": "2023-02-13T05:00:00Z",
      "v": 19481608,
      "vw": 66.435289
    },
    {
      "c": 64.56,
      "h": 66.27,
      "l": 63.71,
      "n": 214100,
      "o": 66.06,
      "t": "2023-02-20T05:00:00Z",
      "v": 19951204,
      "vw": 64.90528
    },
    {
      "c": 64.81,
      "h": 65.58,
      "l": 61.73, // Significant, SSL - Sell Side Liquidity
      "n": 256159,
      "o": 64.76,
      "t": "2023-02-27T05:00:00Z",
      "v": 25895510,
      "vw": 63.552097
    },
    {
      "c": 63.94,
      "h": 65.91,
      "l": 63.63,
      "n": 233625,
      "o": 64.71,
      "t": "2023-03-06T05:00:00Z",
      "v": 19644300,
      "vw": 64.678079
    },
    {
      "c": 67.9,
      "h": 69.05,
      "l": 63.79,
      "n": 318809,
      "o": 63.79,
      "t": "2023-03-13T04:00:00Z",
      "v": 33855760,
      "vw": 67.230145
    },
    {
      "c": 68.59,
      "h": 70.42,
      "l": 66.06,
      "n": 314680,
      "o": 68,
      "t": "2023-03-20T04:00:00Z",
      "v": 27792653,
      "vw": 67.469075
    },
    {
      "c": 69.58,
      "h": 69.755,
      "l": 68.02,
      "n": 244130,
      "o": 68.86,
      "t": "2023-03-27T04:00:00Z",
      "v": 23084368,
      "vw": 68.92255
    }
  ],
  "next_page_token": null,
  "symbol": "SO"
}
```

```rust
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
```
