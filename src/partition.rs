//! Partition theory: integer partitions, Young diagrams, Euler's pentagonal theorem


/// Compute the number of integer partitions of n using dynamic programming.
pub fn partition_number(n: usize) -> u64 {
    if n == 0 { return 1; }
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;
    for k in 1..=n {
        for j in k..=n {
            dp[j] += dp[j - k];
        }
    }
    dp[n]
}

/// Compute partition numbers p(0) through p(n)
pub fn partition_numbers(n: usize) -> Vec<u64> {
    let mut dp = vec![0u64; n + 1];
    dp[0] = 1;
    for k in 1..=n {
        for j in k..=n {
            dp[j] += dp[j - k];
        }
    }
    dp
}

/// Compute p(n) using Euler's pentagonal theorem:
/// p(n) = sum_{k != 0} (-1)^{k+1} * p(n - k(3k-1)/2)
/// where the sum is over generalized pentagonal numbers.
pub fn partition_pentagonal(n: usize) -> u64 {
    if n == 0 { return 1; }
    let mut p = vec![0u64; n + 1];
    p[0] = 1;
    for m in 1..=n {
        let mut sum = 0i64;
        let mut k = 1i64;
        loop {
            // Generalized pentagonal numbers: k*(3k-1)/2 and k*(3k+1)/2
            let g1 = (k * (3 * k - 1)) as usize / 2;
            let g2 = (k * (3 * k + 1)) as usize / 2;
            if g1 > m { break; }
            let sign = if k % 2 == 1 { 1i64 } else { -1i64 };
            sum += sign * p[m - g1] as i64;
            if g2 <= m {
                sum += sign * p[m - g2] as i64;
            }
            k += 1;
        }
        p[m] = sum as u64;
    }
    p[n]
}

/// Generate all integer partitions of n (as vectors of parts in non-increasing order).
pub fn partitions(n: u64) -> Vec<Vec<u64>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    generate_partitions(n, n, &mut current, &mut result);
    result
}

fn generate_partitions(remaining: u64, max_part: u64, current: &mut Vec<u64>, result: &mut Vec<Vec<u64>>) {
    if remaining == 0 {
        result.push(current.clone());
        return;
    }
    let mut part = max_part.min(remaining);
    while part >= 1 {
        current.push(part);
        generate_partitions(remaining - part, part, current, result);
        current.pop();
        part -= 1;
    }
}

/// Young diagram represented as a partition (Ferrers diagram).
/// The conjugate (transpose) of a partition is obtained by flipping rows and columns.
pub fn conjugate_partition(partition: &[u64]) -> Vec<u64> {
    if partition.is_empty() { return vec![]; }
    let _rows = partition.len();
    let cols = partition[0] as usize;
    let mut result = vec![0u64; cols];
    for &row_len in partition {
        for j in 0..row_len as usize {
            result[j] += 1;
        }
    }
    result
}

/// Count partitions of n into at most k parts (equals partitions of n with largest part ≤ k)
pub fn restricted_partitions(n: u64, k: u64) -> u64 {
    let n = n as usize;
    let k = k as usize;
    let mut dp = vec![vec![0u64; k + 1]; n + 1];
    for j in 0..=k {
        dp[0][j] = 1;
    }
    for i in 1..=n {
        for j in 1..=k {
            dp[i][j] = dp[i][j - 1];
            if i >= j {
                dp[i][j] += dp[i - j][j];
            }
        }
    }
    dp[n][k]
}

/// Distinct partitions: partitions into distinct parts
pub fn distinct_partitions(n: u64) -> Vec<Vec<u64>> {
    let all = partitions(n);
    all.into_iter().filter(|p| {
        for i in 1..p.len() {
            if p[i] == p[i - 1] { return false; }
        }
        true
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_number() {
        assert_eq!(partition_number(0), 1);
        assert_eq!(partition_number(1), 1);
        assert_eq!(partition_number(2), 2);
        assert_eq!(partition_number(3), 3);
        assert_eq!(partition_number(4), 5);
        assert_eq!(partition_number(5), 7);
        assert_eq!(partition_number(10), 42);
        assert_eq!(partition_number(20), 627);
    }

    #[test]
    fn test_partition_pentagonal() {
        // Should match partition_number
        for n in 0..=25 {
            assert_eq!(partition_pentagonal(n), partition_number(n), "Mismatch at n={}", n);
        }
    }

    #[test]
    fn test_partitions_count() {
        for n in 0u64..=10 {
            assert_eq!(partitions(n).len() as u64, partition_number(n as usize));
        }
    }

    #[test]
    fn test_conjugate_partition() {
        // Conjugate of [5,3,1] is [3,2,2,1,1]
        assert_eq!(conjugate_partition(&[5, 3, 1]), vec![3, 2, 2, 1, 1]);
        // Conjugate of [4,2,1] is [3,2,1,1]
        assert_eq!(conjugate_partition(&[4, 2, 1]), vec![3, 2, 1, 1]);
        // Self-conjugate: [3,2,1] → [3,2,1]
        assert_eq!(conjugate_partition(&[3, 2, 1]), vec![3, 2, 1]);
    }

    #[test]
    fn test_restricted_partitions() {
        // Partitions of 5 into at most 2 parts: 5, 4+1, 3+2 → 3
        assert_eq!(restricted_partitions(5, 2), 3);
        assert_eq!(restricted_partitions(5, 5), 7); // Same as p(5)
    }

    #[test]
    fn test_distinct_partitions() {
        // Partitions of 5 into distinct parts: 5, 4+1, 3+2 → 3
        assert_eq!(distinct_partitions(5).len(), 3);
    }
}
