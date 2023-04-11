pub fn relative_strength_index(spread: &[f32], period: usize) -> Vec<f32> {
    // How many times can this be looped through
    let iterations = spread.len() / period;

    for i in 0..iterations {
        let start = period * i;
        let end = period * (i + 1);
        let target_spread = &spread[start..end];

        let mut gains = 0.0;
        let mut losses = 0.0;

    }

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

        let result = relative_strength_index(&data, 14);
        dbg!(&result);
        let expect = vec![23.0, 22.0];

        assert_eq!(result, expect);
    }
}
