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
fn main() {
    let sma = simple_moving_average(&[23.2, 26.9, 32.88, 43.321, 20.0]);
    println!("{sma}");
}
