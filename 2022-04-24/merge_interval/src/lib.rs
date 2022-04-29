use std::cmp;

pub fn merge_intervals(mut intervals: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut merged_intervals = vec![];
    while !intervals.is_empty() {
        let mut next_interval = intervals.remove(0);
        let mut temp_vec: Vec<(u64, u64)> = vec![];
        for x in intervals {
            if intervals_overlap(&next_interval, &x) {
                next_interval = (cmp::min(next_interval.0, x.0), cmp::max(next_interval.1, x.1));
            } else {
                temp_vec.push(x);
            }
        }
        merged_intervals.push(next_interval);
        intervals = temp_vec;
    }
    merged_intervals
}

pub fn intervals_overlap(first: &(u64, u64), second: &(u64, u64)) -> bool {
    !(first.1 < second.0 || first.0 > second.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        assert_eq!(merge_intervals(vec![(1, 4), (2, 6), (8, 10), (15, 20)]), vec![(1, 6), (8, 10), (15, 20)]);
        assert_eq!(merge_intervals(vec![(1, 2), (2, 7)]), vec![(1, 7)]);
    }
}
