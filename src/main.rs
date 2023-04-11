use river::{bar::BarSet, Client, Query};
use std::collections::LinkedList;

pub fn simple_moving_average(spread: &[f32]) -> f32 {
    let mut sum = 0.0;
    for number in spread {
        sum += number
    }
    sum / spread.len() as f32
}

pub fn exponential_moving_average(spread: &[f32], period: usize) -> f32 {
    let mut ema = 0.0;
    let mut alpha = 2.0 / (period as f32 + 1.0);

    for i in 0..period {
        ema += spread[i];
    }
    ema /= period as f32;

    for i in period..spread.len() {
        ema = spread[i] * alpha + ema * (1.0 - alpha);
    }

    ema
}

fn standard_deviation(values: &[f32]) -> f32 {
    let length = values.len() as f32;
    let mean = values.iter().sum::<f32>() / length;
    let variance = values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f32>()
        / (length - 1.0);
    variance.sqrt()
}

fn relative_strength_index(prices: &[f32], period: usize) -> Vec<f32> {
    let mut gain_sum = 0.0;
    let mut loss_sum = 0.0;
    let mut rsi_values = Vec::new();
    let mut previous_price = prices[0];

    for (i, &price) in prices.iter().enumerate() {
        if i > 0 {
            let diff = price - previous_price;
            if diff > 0.0 {
                gain_sum += diff;
            } else {
                loss_sum -= diff;
            }
            if i >= period {
                let avg_gain = gain_sum / period as f32;
                let avg_loss = loss_sum / period as f32;
                let rs = if avg_loss == 0.0 { 0.0 } else { avg_gain / avg_loss };
                let rsi = 100.0 - (100.0 / (1.0 + rs));
                rsi_values.push(rsi);
                // Update gain/loss sum for the next period
                gain_sum -= prices[i - period] - previous_price;
                loss_sum -= -prices[i - period] + previous_price;
            }
        }
        previous_price = price;
    }
    rsi_values
}

fn moving_avg_convergent_divergent(spread: &[f32]) -> (Vec<f32>, f32) {
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

pub fn bollinger_bands(spread: &[f32]) -> (f32, f32, f32) {
    let mut upper_band = 0.0;
    let mut lower_band = 0.0;
    let mut middle_band = 0.0;

    // * Period has to be half the amount of data points.
    let period: usize = spread.len() / 2;

    for i in period..spread.len() {
        // * Progressivly "climb up" the arrray one value at a time
        let offset = &i - &period;
        let prices = &spread[offset..spread.len()];
        let mean = simple_moving_average(&prices);
        let std_dev = standard_deviation(&prices);

        // * Get the plot point of this current target slice
        let k = std_dev * 2.0;
        let upper_plot = mean + k;
        let bottom_plot = mean - k;

        // * When the last number is hit, set the values
        if spread.len() - &i == 1 {
            upper_band = upper_plot;
            middle_band = mean;
            lower_band = bottom_plot;
        }
    }

    (upper_band, middle_band, lower_band)
}

fn main() {
    let data = vec![
        25.1, 25.48, 25.34, 25.46, 25.04, 25.54, 24.49, 24.51, 22.54, 22.64, 23.54,
    27.01, 27.25, 26.66, 29.14, 29.68, 29.15, 28.55, 28.82, 28.67, 29.42, 30.19,
    30.21, 30.29, 29.78, 30.47, 28.9, 27.95, 28.3, 28.4, 27.45, 27.01, 25.29,
    25.12, 26.485
    ];
    let test = moving_avg_convergent_divergent(&data);
    println!("{:?}", test);
}
