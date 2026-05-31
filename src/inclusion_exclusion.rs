//! Inclusion-Exclusion principle

/// Apply inclusion-exclusion: given sets of sizes |A_i|,
/// and intersection sizes for all subsets, compute |A_1 ∪ A_2 ∪ ... ∪ A_n|.
///
/// The `intersection_sizes` map should contain, for each non-empty subset represented
/// as a bitmask, the size of the intersection of those sets.
pub fn inclusion_exclusion(intersection_sizes: &[u64]) -> u64 {
    // intersection_sizes[i] is the size of the intersection of sets in the bitmask (1-indexed by bit)
    // intersection_sizes[0] = universe size (unused here, but could be)
    // Actually let's use a simpler API: pass a slice where index = bitmask of which sets are included
    let n = intersection_sizes.len();
    let mut result = 0u64;
    for mask in 1..n {
        let bits_set = (mask as u32).count_ones();
        if bits_set % 2 == 1 {
            result += intersection_sizes[mask];
        } else {
            result -= intersection_sizes[mask];
        }
    }
    result
}

/// Classic application: count integers from 1..=n that are NOT divisible by any of the given primes.
/// Uses inclusion-exclusion.
pub fn count_coprimes(n: u64, primes: &[u64]) -> u64 {
    let m = primes.len();
    let mut count = 0u64;
    for mask in 1u32..(1 << m) {
        let mut product = 1u64;
        let mut bits = 0u32;
        for i in 0..m {
            if mask & (1 << i) != 0 {
                bits += 1;
                product *= primes[i as usize];
                if product > n { break; }
            }
        }
        if product <= n {
            let divisible = n / product;
            if bits % 2 == 1 {
                count += divisible;
            } else {
                count -= divisible;
            }
        }
    }
    n - count
}

/// Derangements: !n = n! * sum_{k=0}^{n} (-1)^k / k!
/// Number of permutations with no fixed points.
pub fn derangements(n: u64) -> u64 {
    if n == 0 { return 1; }
    if n == 1 { return 0; }
    // Use integer recurrence: D(n) = (n-1) * (D(n-1) + D(n-2))
    let mut d0 = 1u64;
    let mut d1 = 0u64;
    for i in 2..=n {
        let d2 = (i - 1) * (d1 + d0);
        d0 = d1;
        d1 = d2;
    }
    d1
}

/// Euler's totient function φ(n): count of integers in [1, n] coprime to n.
pub fn euler_totient(mut n: u64) -> u64 {
    let mut result = n;
    let mut p = 2u64;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 { n /= p; }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inclusion_exclusion_two_sets() {
        // |A| = 10, |B| = 15, |A∩B| = 3 → |A∪B| = 10 + 15 - 3 = 22
        // Bitmask: bit0=A, bit1=B
        let mut sizes = vec![0u64; 4];
        sizes[0b01] = 10; // |A|
        sizes[0b10] = 15; // |B|
        sizes[0b11] = 3;  // |A∩B|
        assert_eq!(inclusion_exclusion(&sizes), 22);
    }

    #[test]
    fn test_inclusion_exclusion_three_sets() {
        // |A|=10, |B|=10, |C|=10, |A∩B|=3, |A∩C|=3, |B∩C|=3, |A∩B∩C|=1
        // = 10+10+10 - 3-3-3 + 1 = 22
        // Bitmask: bit0=A, bit1=B, bit2=C
        let mut sizes = vec![0u64; 8];
        sizes[0b001] = 10; // |A|
        sizes[0b010] = 10; // |B|
        sizes[0b100] = 10; // |C|
        sizes[0b011] = 3;  // |A∩B|
        sizes[0b101] = 3;  // |A∩C|
        sizes[0b110] = 3;  // |B∩C|
        sizes[0b111] = 1;  // |A∩B∩C|
        assert_eq!(inclusion_exclusion(&sizes), 22);
    }

    #[test]
    fn test_count_coprimes() {
        // Euler totient φ(30) = count of 1..30 not divisible by 2,3,5
        // 30 * (1-1/2) * (1-1/3) * (1-1/5) = 30 * 1/2 * 2/3 * 4/5 = 8
        assert_eq!(count_coprimes(30, &[2, 3, 5]), 8);
    }

    #[test]
    fn test_derangements() {
        assert_eq!(derangements(0), 1);
        assert_eq!(derangements(1), 0);
        assert_eq!(derangements(2), 1);
        assert_eq!(derangements(3), 2);
        assert_eq!(derangements(4), 9);
        assert_eq!(derangements(5), 44);
    }

    #[test]
    fn test_euler_totient() {
        assert_eq!(euler_totient(1), 1);
        assert_eq!(euler_totient(6), 2);
        assert_eq!(euler_totient(10), 4);
        assert_eq!(euler_totient(30), 8);
        assert_eq!(euler_totient(7), 6); // prime
    }
}
