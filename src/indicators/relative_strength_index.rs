pub fn relative_strength_index(prices: &[f32], period: usize) -> Vec<f32> {
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
