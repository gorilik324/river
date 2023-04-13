use crate::indicators::{simple_moving_average, standard_deviation};

pub fn bollinger_bands(spread: &[f32]) -> (f32, f32, f32) {
    if spread.len() < 4 {
        panic!("Not Enough Data Points For Bollinger Bands")
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bollinger_bands_result() {
        // Data from XLF, weekly bars, Mon Aug 08, 22 -> Mon Apr 10, 23
        // ... used trading view chart and indicator.
        let data = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
            36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
        ];
        let result = bollinger_bands(&data);
        dbg!(&result);
        let expect = (38.164547, 34.314735, 30.464926);

        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic(expected = "Not Enough Data Points For Bollinger Bands")]
    fn test_ema_error() {
        let data: Vec<f32> = vec![
            10.0, 12.0, 13.0
        ];

        let result = bollinger_bands(&data);
    }
}
