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
fn main() {
    let data = vec![17.76, 17.48, 16.65, 17.7, 17.21, 17.49, 18.01, 18.85, 19.25, 18.78];
    let period = 3;
    let test = exponential_moving_average(&data, period);
    println!("{test}");
}
