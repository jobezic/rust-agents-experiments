pub fn calculate_average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }

    let sum: f64 = numbers.iter().sum();
    Some(sum / numbers.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average_normal() {
        let numbers = [2.0, 4.0, 6.0, 8.0];
        assert_eq!(calculate_average(&numbers), Some(5.0));
    }

    #[test]
    fn test_calculate_average_single_element() {
        let numbers = [42.0];
        assert_eq!(calculate_average(&numbers), Some(42.0));
    }

    #[test]
    fn test_calculate_average_empty() {
        let numbers: [f64; 0] = [];
        assert_eq!(calculate_average(&numbers), None);
    }
}