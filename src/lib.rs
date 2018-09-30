#[cfg(test)]
mod tests {
    use mean;
    use sum;
    use variance;
    #[test]
    fn test_mean() {
        assert_eq!(mean(&vec![1.0, 2.0, 3.0]), 2.0);
    }
    #[test]
    fn test_sum() {
        assert_eq!(sum(&vec![1.0, 2.0, 3.0]), 6.0);
    }
    #[test]
    fn test_variance() {
        assert_eq!(variance(&vec![0.0, 1.0]), 0.25);
    }
}


pub fn mean(values: &Vec<f64>) -> f64 {
    sum(values) / (values.len() as f64)
}

fn sum(values: &Vec<f64>) -> f64 {
    values.iter().fold(0.0, |a, b| a + b)
}

pub fn variance(values: &Vec<f64>) -> f64 {
    let mean = mean(values);
    values
        .iter()
        .map(|x| (x - mean).powf(2.0))
        .fold(0.0, |a, b| a + b) / (values.len() as f64)
}

pub fn stdev(values: &Vec<f64>) -> f64 {
    variance(values).powf(0.5)
}
    
