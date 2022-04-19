pub fn sum_sublist(integers: &[u64], window_size: usize) -> &[u64] {
    if integers.len() <= window_size {
        return integers;
    }
    let mut max_slice: &[u64]  = &[];
    let mut current_max_value = 0;
    for window in integers.windows(window_size) {
        let sum = window.iter().sum();
        if sum > current_max_value {
            current_max_value = sum;
            max_slice = window;
        }
    }
    max_slice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sublist() {
        assert_eq!(sum_sublist(&[1, 2, 3, 4, 5], 3), &[3, 4, 5]);
        assert_eq!(sum_sublist(&[9, 2, 3, 4, 5], 3), &[9, 2, 3]);
        assert_eq!(sum_sublist(&[3, 1, 4, 1, 5, 9, 2, 6], 3), &[9, 2, 6]);
    }
}
