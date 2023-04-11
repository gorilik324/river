use crate::indicators::exponential_moving_average;

pub fn moving_avg_convergent_divergent(spread: &[f32]) -> (Vec<f32>, f32) {
    let mut macd_line = Vec::new();

    for i in 0..9 {
      // * Remove the last element in array on each loop through.
      let slice_length = spread.len() - i;

      let target_prices = &spread[0..slice_length];

      let short_period = exponential_moving_average(&target_prices[target_prices.len() - 12..], 12);

      let long_period = exponential_moving_average(&target_prices[target_prices.len() - 24..], 24);

      let diff = short_period - long_period;

      macd_line.insert(0, diff);
    }

    let signal_line = exponential_moving_average(&macd_line, 9);
    return (macd_line, signal_line);
}
