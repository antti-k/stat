#[cfg(test)]
mod tests {
    use mean;
    use sum;
    #[test]
    fn test_mean() {
        assert_eq!(mean(&vec![1.0, 2.0, 3.0]), 2.0);
    }
    #[test]
    fn test_sum() {
        assert_eq!(sum(&vec![1.0, 2.0, 3.0]), 6.0);
    }
}


pub fn mean(values: &Vec<f64>) -> f64 {
    sum(values) / (values.len() as f64)
}

pub fn sum(values: &Vec<f64>) -> f64 {
    values.iter().fold(0.0, |a, b| a + b)
}
