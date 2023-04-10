use river::{Client, Query, bar::BarSet};

pub fn simple_moving_average(spread: &[f32]) -> f32 {
    // Sum the spread
    let mut sum = 0.0;
    for number in spread {
        sum += number
    }

    // Divide by the length of the spread
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
    let variance = values.iter().map(|value| (value - mean).powi(2)).sum::<f32>() / (length - 1.0);
    variance.sqrt()
}

pub fn bollinger_bands(spread: &[f32]) -> (f32, f32, f32) {
    // * Get 40 days of data, need first 20 for first data point.
    let mut upper_band = 0.0;
    let mut lower_band = 0.0;
    let mut middle_band = 0.0;
    let period: usize = spread.len() / 2;

    for i in period..spread.len() {
        // * Progressivly "climb up" the arrray one value at a time
        let offset = &i - &period;
        let prices = &spread[offset..20];
        let mean = simple_moving_average(&prices);
        let std_dev = standard_deviation(&prices);

        // * Get the plot point of this current target slice
        let k = std_dev * 2.0;

        let upper_plot = mean + k;
        let bottom_plot = mean - k;

        // * When the last number is hit. Used to calculate BBtrend.
        if spread.len() - &i == 1 {
          upper_band = upper_plot;
          middle_band = mean;
          lower_band = bottom_plot;
        }
    }
    (upper_band, middle_band, lower_band)
}

fn main() {
    let data = vec![17.76, 17.48, 16.65, 17.7, 17.21, 17.49, 18.01, 18.85, 19.25, 18.78,17.76, 17.48, 16.65, 17.7, 17.21, 17.49, 18.01, 18.85, 19.25, 18.78 ];
    let test = bollinger_bands(&data);
    println!("{:?}", test);
}
