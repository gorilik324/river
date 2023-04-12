pub fn relative_strength_index(spread: &[f32]) -> Vec<f32> {
    let period = 14;
    let mut gains = Vec::new();
    let mut losses = Vec::new();
    let mut last_price = 0.0;
    let mut rsi_values = Vec::new();

    for (i, price) in spread.iter().enumerate() {
        if i == 0 {
            last_price = *price;
            continue;
        }

        let change = price - last_price;

        if change > 0.0 {
            gains.push(change);
            losses.push(0.0);
        } else if change < 0.0 {
            gains.push(0.0);
            losses.push(change.abs());
        } else {
            gains.push(0.0);
            losses.push(0.0);
        }

        if i >= period {
            let avg_gain = gains[i - period..i].iter().sum::<f32>() / period as f32;
            let avg_loss = losses[i - period..i].iter().sum::<f32>() / period as f32;

            let rs = if avg_loss == 0.0 {
                1.0
            } else {
                avg_gain / avg_loss
            };

            let rsi = 100.0 - (100.0 / (1.0 + rs));
            rsi_values.push(rsi);
        }

        last_price = *price;
    }

    rsi_values
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
        let expect = vec![23.0, 22.0];

        assert_eq!(result, expect);
    }
}
