pub fn simple_moving_average(spread: &[f32]) -> f32 {
    let mut sum = 0.0;
    for number in spread {
        sum += number
    }
    sum / spread.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma() {
        let data = vec![4.0, 9.0, 15.0, 18.0, 21.0];
        let result = simple_moving_average(&data);
        let expected = 13.4;
        assert_eq!(result, expected);
    }
}
