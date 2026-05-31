//! Pigeonhole principle applications

/// Given n items and k bins, return the minimum maximum load.
/// By pigeonhole, at least one bin has ceil(n/k) items.
pub fn min_max_load(n: u64, k: u64) -> u64 {
    if k == 0 { panic!("Cannot have 0 bins"); }
    (n + k - 1) / k
}

/// Verify the Erdős–Szekeres theorem: any sequence of n distinct numbers
/// contains an increasing or decreasing subsequence of length at least ceil(sqrt(n)).
pub fn erdos_szekeres_bound(n: u64) -> u64 {
    if n == 0 { return 0; }
    let s = (n as f64).sqrt().ceil() as u64;
    s
}

/// Find the longest increasing subsequence length using patience sorting.
/// Returns (lis_length, sequence)
pub fn longest_increasing_subsequence(seq: &[i64]) -> usize {
    if seq.is_empty() { return 0; }
    let mut piles: Vec<i64> = Vec::new();
    for &x in seq {
        let pos = piles.partition_point(|&p| p < x);
        if pos == piles.len() {
            piles.push(x);
        } else {
            piles[pos] = x;
        }
    }
    piles.len()
}

/// Pigeonhole application: given n+1 integers in [1, n], find a duplicate.
/// Returns the first duplicate found, or None.
pub fn find_duplicate(nums: &[u64]) -> Option<u64> {
    let n = nums.len().saturating_sub(1);
    let mut seen = vec![false; n + 1];
    for &x in nums {
        let x = x as usize;
        if x <= n {
            if seen[x] { return Some(x as u64); }
            seen[x] = true;
        }
    }
    None
}

/// Birthday problem: given d possible values, find the smallest n such that
/// the probability of at least one collision exceeds threshold.
pub fn birthday_collision_threshold(d: u64, threshold: f64) -> u64 {
    let mut prob_no_collision = 1.0_f64;
    let d_f = d as f64;
    for n in 1..=d {
        prob_no_collision *= (d_f - (n as f64 - 1.0)) / d_f;
        if 1.0 - prob_no_collision >= threshold {
            return n;
        }
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_load() {
        assert_eq!(min_max_load(10, 3), 4); // ceil(10/3) = 4
        assert_eq!(min_max_load(9, 3), 3);
        assert_eq!(min_max_load(1, 5), 1);
    }

    #[test]
    fn test_erdos_szekeres_bound() {
        assert_eq!(erdos_szekeres_bound(1), 1);
        assert_eq!(erdos_szekeres_bound(4), 2);
        assert_eq!(erdos_szekeres_bound(10), 4);
        assert_eq!(erdos_szekeres_bound(16), 4);
        assert_eq!(erdos_szekeres_bound(17), 5);
    }

    #[test]
    fn test_lis() {
        assert_eq!(longest_increasing_subsequence(&[10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(longest_increasing_subsequence(&[1, 2, 3, 4, 5]), 5);
        assert_eq!(longest_increasing_subsequence(&[5, 4, 3, 2, 1]), 1);
    }

    #[test]
    fn test_find_duplicate() {
        assert_eq!(find_duplicate(&[1, 3, 4, 2, 2]), Some(2));
        assert_eq!(find_duplicate(&[3, 1, 3, 4, 2]), Some(3));
        assert_eq!(find_duplicate(&[1, 2, 3, 4]), None);
    }

    #[test]
    fn test_birthday_collision() {
        // Classic birthday problem: with 365 days, ~23 people needed for >50% chance
        let n = birthday_collision_threshold(365, 0.5);
        assert!(n >= 22 && n <= 24);
    }
}
