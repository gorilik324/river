use crate::indicators::{standard_deviation, simple_moving_average};

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
