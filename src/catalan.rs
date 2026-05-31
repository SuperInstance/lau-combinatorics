//! Catalan numbers and related structures (Dyck paths, binary trees, parentheses)

/// Compute the n-th Catalan number: C_n = (2n)! / ((n+1)! * n!) = binomial(2n, n) / (n+1)
pub fn catalan(n: u64) -> u64 {
    if n == 0 { return 1; }
    crate::basic::binomial(2 * n, n) / (n + 1)
}

/// Sequence of Catalan numbers up to n
pub fn catalan_sequence(n: usize) -> Vec<u64> {
    (0..=n as u64).map(catalan).collect()
}

/// Count Dyck paths of semilength n (lattice paths from (0,0) to (2n,0)
/// with steps (1,+1) and (1,-1) that never go below y=0)
pub fn dyck_paths(n: u64) -> u64 {
    catalan(n)
}

/// Count the number of full binary trees with n+1 leaves (n internal nodes)
pub fn full_binary_trees(n: u64) -> u64 {
    catalan(n)
}

/// Count valid parentheses strings with n pairs
pub fn valid_parentheses(n: u64) -> u64 {
    catalan(n)
}

/// Count non-crossing partitions of an n-element set arranged on a circle
pub fn non_crossing_partitions(n: u64) -> u64 {
    catalan(n)
}

/// Count ways to triangulate a convex (n+2)-gon
pub fn triangulations(n: u64) -> u64 {
    catalan(n)
}

/// Count binary search trees with n nodes
pub fn binary_search_trees(n: u64) -> u64 {
    catalan(n)
}

/// Ballot problem: number of sequences of a votes for A and b votes for B (a > b)
/// where A is always strictly ahead of B.
/// = (a-b)/(a+b) * C(a+b, a)
pub fn ballot_number(a: u64, b: u64) -> u64 {
    if a <= b { return 0; }
    crate::basic::binomial(a + b, a) * (a - b) / (a + b)
}

/// Narayana numbers: N(n, k) = C(n, k) * C(n, k-1) / n
/// Counts Dyck paths of semilength n with exactly k peaks
pub fn narayana(n: u64, k: u64) -> u64 {
    if n == 0 || k == 0 || k > n { return 0; }
    crate::basic::binomial(n, k) * crate::basic::binomial(n, k - 1) / n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalan_sequence_values() {
        let seq = catalan_sequence(10);
        assert_eq!(seq, vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796]);
    }

    #[test]
    fn test_catalan_identity() {
        // sum of C_i * C_{n-1-i} for i=0..n = C_n (Catalan recurrence)
        for n in 0..=8 {
            let mut sum = 0u64;
            for i in 0..=n {
                sum += catalan(i) * catalan(n - i);
            }
            // This gives C_{n+1}
            assert_eq!(sum, catalan(n + 1));
        }
    }

    #[test]
    fn test_dyck_paths() {
        assert_eq!(dyck_paths(0), 1);
        assert_eq!(dyck_paths(1), 1);
        assert_eq!(dyck_paths(2), 2);
        assert_eq!(dyck_paths(3), 5);
        assert_eq!(dyck_paths(4), 14);
    }

    #[test]
    fn test_triangulations() {
        // Pentagon (n=3) has 5 triangulations
        assert_eq!(triangulations(3), 5);
        // Hexagon (n=4) has 14
        assert_eq!(triangulations(4), 14);
    }

    #[test]
    fn test_valid_parentheses() {
        assert_eq!(valid_parentheses(3), 5); // ((())), (()()), (())(), ()(()), ()()()
        assert_eq!(valid_parentheses(4), 14);
    }

    #[test]
    fn test_ballot_number() {
        // Ballot(3,2) = C(5,3)*(3-2)/(3+2) = 10/5 = 2
        assert_eq!(ballot_number(3, 2), 2);
        // Ballot(3,1) = C(4,3)*(3-1)/(3+1) = 4*2/4 = 2
        assert_eq!(ballot_number(3, 1), 2);
    }

    #[test]
    fn test_narayana() {
        // N(n, k) = C(n, k) * C(n, k-1) / n
        // N(4,1) = C(4,1)*C(4,0)/4 = 4/4 = 1
        // N(4,2) = C(4,2)*C(4,1)/4 = 24/4 = 6
        // N(4,3) = C(4,3)*C(4,2)/4 = 24/4 = 6
        // N(4,4) = C(4,4)*C(4,3)/4 = 4/4 = 1
        assert_eq!(narayana(4, 1), 1);
        assert_eq!(narayana(4, 2), 6);
        assert_eq!(narayana(4, 3), 6);
        assert_eq!(narayana(4, 4), 1);
        // Sum of N(n,k) for k=1..n = C_n
        assert_eq!(narayana(4, 1) + narayana(4, 2) + narayana(4, 3) + narayana(4, 4), 14);
    }
}
