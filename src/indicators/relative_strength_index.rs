fn calc_mean(spread: &[f32]) -> f32 {
    let mut sum = 0.0;
    for i in spread {
        sum += i
    }
    sum / spread.len() as f32
}
pub fn relative_strength_index(spread: &[f32]) -> Vec<f32> {
    // 100 - (100/ 1+(avg_gain/avg_loss))
    let period = 14;
    let mut gains = Vec::new();
    let mut losses = Vec::new();

    // Remove last value, -1, as not to overflow the indexing
    for i in 0..spread.len() - 1 {
        let diff = &spread[i + 1] - &spread[i];
        if diff > 0.0 {
            gains.push(diff);
            losses.push(0.0);
        } else {
            losses.push(diff.abs());
            gains.push(0.0);
        }
    }

    let mut rsi_line = Vec::new();
    let iterations = spread.len() - period;

    for i in 0..iterations {
        let avg_gain = calc_mean(&gains[i..14 + i]);
        let avg_loss = calc_mean(&losses[i..14 + i]);
        let rsi = 100.0 - (100.0 / (1.0 + (avg_gain / avg_loss)));
        rsi_line.push(rsi);
    }

    rsi_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_result() {
        let data: Vec<f32> = vec![
            35.56, 34.96, 33.72, 32.89, 34.36, 33.06, 31.05, 30.36, 30.89, 31.01, 32.19, 34.19,
            33.91, 35.87, 35.37, 36.11, 35.93, 34.53, 33.70, 33.95, 34.20, 35.38, 36.12, 35.35,
            36.25, 36.59, 36.49, 36.39, 35.66, 35.99, 32.93, 30.98, 30.99, 32.15, 31.99, 32.34,
        ];

        let result = relative_strength_index(&data);
        dbg!(&result);
        let expect = vec![
            49.35417, 53.87206, 58.01305, 55.710304, 47.594753, 53.512234, 64.43631, 72.01755,
            72.5237, 67.69983, 66.944916, 61.62791, 62.721905, 53.140102, 51.70388, 49.259266,
            36.338802, 34.605385, 37.34827, 42.254738, 40.41631, 35.794403,
        ];

        assert_eq!(result, expect);
    }
}
