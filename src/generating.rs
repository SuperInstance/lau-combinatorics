//! Generating functions: ordinary and exponential, coefficient extraction

use serde::{Deserialize, Serialize};

/// Ordinary generating function represented as a vector of coefficients.
/// G(x) = a_0 + a_1*x + a_2*x^2 + ...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OGF {
    pub coeffs: Vec<u64>,
}

impl OGF {
    pub fn new(coeffs: Vec<u64>) -> Self {
        Self { coeffs }
    }

    /// Multiply two OGFs (Cauchy product)
    pub fn multiply(&self, other: &OGF) -> OGF {
        let n = self.coeffs.len() + other.coeffs.len() - 1;
        let mut result = vec![0u64; n];
        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                result[i + j] += self.coeffs[i] * other.coeffs[j];
            }
        }
        OGF::new(result)
    }

    /// Extract coefficient of x^n
    pub fn coeff(&self, n: usize) -> u64 {
        self.coeffs.get(n).copied().unwrap_or(0)
    }

    /// Truncate to degree n
    pub fn truncate(&self, n: usize) -> OGF {
        let mut c = self.coeffs.clone();
        c.truncate(n + 1);
        OGF::new(c)
    }

    /// OGF for (1-x)^(-k) = C(n+k-1, k-1) — combinations with repetition
    pub fn negative_binomial(k: u64, terms: usize) -> OGF {
        let coeffs: Vec<u64> = (0..terms)
            .map(|n| crate::basic::binomial(n as u64 + k - 1, k - 1))
            .collect();
        OGF::new(coeffs)
    }

    /// OGF for 1/(1-x) = 1 + x + x^2 + ...
    pub fn geometric(terms: usize) -> OGF {
        OGF::new(vec![1; terms])
    }

    /// OGF for Fibonacci: F(x) = x/(1-x-x^2)
    pub fn fibonacci(terms: usize) -> OGF {
        let mut fib = vec![0u64; terms];
        if terms > 1 { fib[1] = 1; }
        for i in 2..terms {
            fib[i] = fib[i - 1] + fib[i - 2];
        }
        OGF::new(fib)
    }
}

/// Exponential generating function represented as a vector of coefficients.
/// E(x) = a_0 + a_1*x + a_2*x^2/2! + ...
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EGF {
    pub coeffs: Vec<u64>,
}

impl EGF {
    pub fn new(coeffs: Vec<u64>) -> Self {
        Self { coeffs }
    }

    /// Multiply two EGFs (binomial convolution):
    /// c_n = sum_{k=0}^{n} C(n,k) * a_k * b_{n-k}
    pub fn multiply(&self, other: &EGF) -> EGF {
        let n = self.coeffs.len() + other.coeffs.len() - 1;
        let mut result = vec![0u64; n];
        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                if i + j < n {
                    result[i + j] += crate::basic::binomial((i + j) as u64, i as u64)
                        * self.coeffs[i] * other.coeffs[j];
                }
            }
        }
        EGF::new(result)
    }

    /// Extract the coefficient a_n (before dividing by n!)
    pub fn raw_coeff(&self, n: usize) -> u64 {
        self.coeffs.get(n).copied().unwrap_or(0)
    }

    /// EGF for exp(x) = 1 + x + x^2/2! + ... (all coefficients = 1)
    pub fn exponential(terms: usize) -> EGF {
        EGF::new(vec![1; terms])
    }

    /// EGF for B_n (Bell numbers)
    pub fn bell(terms: usize) -> EGF {
        let mut bells = vec![0u64; terms];
        for i in 0..terms {
            bells[i] = crate::basic::bell_number(i as u64);
        }
        EGF::new(bells)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ogf_multiply() {
        // (1 + x)^2 = 1 + 2x + x^2
        let a = OGF::new(vec![1, 1]);
        let b = OGF::new(vec![1, 1]);
        let c = a.multiply(&b);
        assert_eq!(c.coeffs, vec![1, 2, 1]);
    }

    #[test]
    fn test_ogf_negative_binomial() {
        // 1/(1-x)^2 = 1 + 2x + 3x^2 + 4x^3 + ...
        let g = OGF::negative_binomial(2, 5);
        assert_eq!(g.coeff(0), 1);
        assert_eq!(g.coeff(1), 2);
        assert_eq!(g.coeff(2), 3);
        assert_eq!(g.coeff(3), 4);
        assert_eq!(g.coeff(4), 5);
    }

    #[test]
    fn test_ogf_fibonacci() {
        let g = OGF::fibonacci(10);
        assert_eq!(g.coeff(0), 0);
        assert_eq!(g.coeff(1), 1);
        assert_eq!(g.coeff(2), 1);
        assert_eq!(g.coeff(3), 2);
        assert_eq!(g.coeff(4), 3);
        assert_eq!(g.coeff(5), 5);
        assert_eq!(g.coeff(6), 8);
        assert_eq!(g.coeff(7), 13);
    }

    #[test]
    fn test_egf_multiply() {
        // exp(x) * exp(x) = exp(2x), coefficients should be 2^n
        let a = EGF::exponential(5);
        let c = a.multiply(&a);
        assert_eq!(c.raw_coeff(0), 1);  // 2^0 = 1
        assert_eq!(c.raw_coeff(1), 2);  // 2^1 = 2
        assert_eq!(c.raw_coeff(2), 4);  // 2^2 = 4
        assert_eq!(c.raw_coeff(3), 8);  // 2^3 = 8
        assert_eq!(c.raw_coeff(4), 16); // 2^4 = 16
    }

    #[test]
    fn test_egf_bell() {
        let b = EGF::bell(6);
        assert_eq!(b.raw_coeff(0), 1);
        assert_eq!(b.raw_coeff(1), 1);
        assert_eq!(b.raw_coeff(2), 2);
        assert_eq!(b.raw_coeff(3), 5);
        assert_eq!(b.raw_coeff(4), 15);
        assert_eq!(b.raw_coeff(5), 52);
    }

    #[test]
    fn test_ogf_geometric() {
        let g = OGF::geometric(5);
        assert_eq!(g.coeffs, vec![1, 1, 1, 1, 1]);
    }
}
