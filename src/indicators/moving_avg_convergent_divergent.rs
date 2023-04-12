use crate::indicators::exponential_moving_average;

pub fn moving_avg_convergent_divergent(spread: &[f32]) -> (Vec<f32>, f32) {
    if spread.len() < 26 + 9 {
        panic!("Spread Length Is Too Short For MACD")
    }
    let mut macd_line = Vec::new();

    // Using the last 9 values in the spread, fill the MACD vec.
    for i in 0..9 {
        // Need to walk backwards through the spread to get present day MACD line.
        // * Full length, full length - 1, full length -2, etc...
        let target_prices = &spread[0..spread.len() - i];

        // MACD line = 12-period EMA - 26-period EMA
        let short_period = exponential_moving_average(&target_prices, 12);
        let long_period = exponential_moving_average(&target_prices, 26);
        let macd = short_period - long_period;

        // Place value at the start of the vec on each loop. This ensures
        // ... that the most recent MACD point is at the end of the vec.
        macd_line.insert(0, macd);
    }

    let signal_line = exponential_moving_average(&macd_line, 9);
    return (macd_line, signal_line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macd_result() {
        // Data from BTU, weekly bars, Mon Aug 08, 22 -> Mon Apr 10, 23
        // ... used trading view chart and indicator.
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
            36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
        ];

        // used https://www.easycalculation.com/finance/macd.php to test this.
        let result = moving_avg_convergent_divergent(&data);
        dbg!(&result);
        let expect = (
            vec![
                1.1245346,
                1.0470009,
                1.0006447,
                0.70882416,
                0.31655502,
                0.0064086914,
                -0.14411926,
                -0.2731743,
                -0.3432541,
            ],
            0.38260227,
        );

        // These are the values trading view had
        //let expect = (vec![0.6227, 0.5839, 0.5731, 0.3140, -0.0481, -0.3305, -0.4554, -0.5608, -0.6079], -0.1712);
        assert_eq!(result, expect);
    }

    #[test]
    #[should_panic(expected = "Spread Length Is Too Short For MACD")]
    fn test_macd_error() {
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11,
        ];
        let result = moving_avg_convergent_divergent(&data);
    }
}
