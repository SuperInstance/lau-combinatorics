//! Basic counting: permutations, combinations, multinomial coefficients


/// Compute n! (factorial). Returns 1 for n=0.
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Permutations: P(n, k) = n! / (n-k)!
pub fn permutations(n: u64, k: u64) -> u64 {
    if k > n { return 0; }
    (n - k + 1..=n).product()
}

/// Combinations: C(n, k) = n! / (k!(n-k)!)
/// Alias for binomial
pub fn combinations(n: u64, k: u64) -> u64 {
    binomial(n, k)
}

/// Combinations: C(n, k) using multiplicative formula (correct integer arithmetic)
pub fn binomial(n: u64, k: u64) -> u64 {
    if k > n { return 0; }
    let k = k.min(n - k);
    if k == 0 { return 1; }
    let mut result: u64 = 1;
    for i in 0..k {
        // Use wider arithmetic to avoid overflow in intermediate
        result = result * (n - i) / (i + 1);
    }
    result
}

/// Multinomial coefficient: n! / (k1! * k2! * ... * km!)
/// where k1 + k2 + ... + km = n
pub fn multinomial(n: u64, ks: &[u64]) -> u64 {
    let sum: u64 = ks.iter().sum();
    if sum != n { return 0; }
    let mut result = factorial(n);
    for &k in ks {
        result /= factorial(k);
    }
    result
}

/// Falling factorial: n * (n-1) * ... * (n-k+1)
pub fn falling_factorial(n: u64, k: u64) -> u64 {
    if k > n { return 0; }
    (0..k).map(|i| n - i).product()
}

/// Rising factorial: n * (n+1) * ... * (n+k-1)
pub fn rising_factorial(n: u64, k: u64) -> u64 {
    (0..k).map(|i| n + i).product()
}

/// Stars and bars: C(n + k - 1, k - 1) — ways to distribute n identical items into k bins
pub fn stars_and_bars(n: u64, k: u64) -> u64 {
    if k == 0 { return if n == 0 { 1 } else { 0 }; }
    binomial(n + k - 1, k - 1)
}

/// Stirling numbers of the second kind: S(n, k)
/// Number of ways to partition n elements into k non-empty subsets
pub fn stirling_second(n: u64, k: u64) -> u64 {
    if n == 0 && k == 0 { return 1; }
    if n == 0 || k == 0 { return 0; }
    if k > n { return 0; }
    if k == 1 { return 1; }
    if k == n { return 1; }
    // S(n, k) = k * S(n-1, k) + S(n-1, k-1)
    let n = n as usize;
    let k = k as usize;
    let mut dp = vec![0u64; k + 1];
    dp[0] = 1;
    for i in 1..=n {
        let mut new_dp = vec![0u64; k + 1];
        for j in 1..=k.min(i) {
            new_dp[j] = j as u64 * dp[j] + dp[j - 1];
        }
        dp = new_dp;
    }
    dp[k]
}

/// Bell number B(n): total number of partitions of an n-element set
pub fn bell_number(n: u64) -> u64 {
    (0..=n).map(|k| stirling_second(n, k)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(5, 3), 60);
        assert_eq!(permutations(10, 0), 1);
        assert_eq!(permutations(4, 4), 24);
        assert_eq!(permutations(3, 5), 0);
    }

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 2), 10);
        assert_eq!(binomial(10, 3), 120);
        assert_eq!(binomial(0, 0), 1);
        assert_eq!(binomial(5, 0), 1);
        assert_eq!(binomial(5, 5), 1);
        assert_eq!(binomial(6, 3), 20);
        assert_eq!(binomial(7, 2), 21);
        assert_eq!(binomial(10, 5), 252);
    }

    #[test]
    fn test_multinomial() {
        // (a+b+c)^2 coefficient of a^0 b^1 c^1 = 2
        assert_eq!(multinomial(2, &[0, 1, 1]), 2);
        // 6! / (2!2!2!) = 720 / 8 = 90
        assert_eq!(multinomial(6, &[2, 2, 2]), 90);
        // 5! / (2!3!) = 10
        assert_eq!(multinomial(5, &[2, 3]), 10);
    }

    #[test]
    fn test_stars_and_bars() {
        // Distribute 5 items into 3 bins: C(7,2) = 21
        assert_eq!(stars_and_bars(5, 3), 21);
        assert_eq!(stars_and_bars(0, 3), 1);
        assert_eq!(stars_and_bars(3, 1), 1);
    }

    #[test]
    fn test_stirling_second() {
        assert_eq!(stirling_second(4, 2), 7);
        assert_eq!(stirling_second(5, 3), 25);
        assert_eq!(stirling_second(3, 1), 1);
        assert_eq!(stirling_second(3, 3), 1);
    }

    #[test]
    fn test_bell_number() {
        assert_eq!(bell_number(0), 1);
        assert_eq!(bell_number(1), 1);
        assert_eq!(bell_number(3), 5);
        assert_eq!(bell_number(4), 15);
        assert_eq!(bell_number(5), 52);
    }

    #[test]
    fn test_falling_factorial() {
        assert_eq!(falling_factorial(5, 3), 60);
        assert_eq!(falling_factorial(10, 0), 1);
    }

    #[test]
    fn test_rising_factorial() {
        assert_eq!(rising_factorial(3, 4), 360);
        assert_eq!(rising_factorial(5, 0), 1);
    }
}
