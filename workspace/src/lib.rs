/// Calculates the arithmetic average of a slice of f64 numbers.
///
/// Returns `None` if the slice is empty, otherwise returns `Some(average)`.
pub fn calculate_average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }

    let sum: f64 = numbers.iter().sum();
    Some(sum / (numbers.len() as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_slice() {
        assert_eq!(calculate_average(&[]), None);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(calculate_average(&[5.0]), Some(5.0));
    }

    #[test]
    fn test_multiple_elements() {
        let nums = [2.0, 4.0, 6.0, 8.0];
        assert_eq!(calculate_average(&nums), Some(5.0));
    }

    #[test]
    fn test_negative_numbers() {
        let nums = [-10.0, 0.0, 10.0];
        assert_eq!(calculate_average(&nums), Some(0.0));
    }

    #[test]
    fn test_fractional_results() {
        let nums = [1.0, 2.0];
        assert_eq!(calculate_average(&nums), Some(1.5));
    }
}
