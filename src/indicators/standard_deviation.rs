pub fn standard_deviation(values: &[f32]) -> f32 {
    let length = values.len() as f32;
    let mean = values.iter().sum::<f32>() / length;
    let variance = values
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f32>()
        / (length - 1.0);
    variance.sqrt()
}
