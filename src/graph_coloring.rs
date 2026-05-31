//! Graph coloring: chromatic polynomial, greedy coloring

use serde::{Deserialize, Serialize};

/// Simple undirected graph represented as adjacency list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub n: usize,
    pub adj: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Graph {
            n,
            adj: vec![vec![]; n],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u != v && !self.adj[u].contains(&v) {
            self.adj[u].push(v);
            self.adj[v].push(u);
        }
    }

    /// Compute the chromatic polynomial P(G, k) using deletion-contraction.
    /// Returns a vector of coefficients where result[i] is the coefficient of k^i.
    pub fn chromatic_polynomial(&self) -> Vec<i64> {
        self._chromatic_poly_internal()
    }

    fn _chromatic_poly_internal(&self) -> Vec<i64> {
        let n = self.n;

        // Base case: no edges → k^n
        let edge_count: usize = self.adj.iter().map(|a| a.len()).sum::<usize>() / 2;
        if edge_count == 0 {
            let mut poly = vec![0i64; n + 1];
            poly[n] = 1;
            return poly;
        }

        // Find an edge
        let mut edge = None;
        'outer: for u in 0..n {
            for &v in &self.adj[u] {
                if u < v {
                    edge = Some((u, v));
                    break 'outer;
                }
            }
        }

        let (u, v) = edge.unwrap();

        // Deletion: remove edge u-v
        let mut g_del = self.clone();
        g_del.adj[u].retain(|&x| x != v);
        g_del.adj[v].retain(|&x| x != u);

        // Contraction: merge u and v
        let g_cont = self.contract(u, v);

        let p_del = g_del._chromatic_poly_internal();
        let p_cont = g_cont._chromatic_poly_internal();

        // P(G) = P(G-e) - P(G/e)
        poly_sub(&p_del, &p_cont)
    }

    fn contract(&self, u: usize, v: usize) -> Graph {
        let min_uv = u.min(v);
        let max_uv = u.max(v);
        let new_n = self.n - 1;

        // Map old vertices to new: vertices > max_uv shift down by 1,
        // max_uv merges into min_uv
        let mapping: Vec<usize> = (0..self.n).map(|i| {
            if i == max_uv { min_uv }
            else if i > max_uv { i - 1 }
            else { i }
        }).collect();

        let mut g = Graph::new(new_n);
        for i in 0..self.n {
            for &j in &self.adj[i] {
                if i < j && !(i == min_uv && j == max_uv || i == max_uv && j == min_uv) {
                    g.add_edge(mapping[i], mapping[j]);
                }
            }
        }
        g
    }

    /// Evaluate chromatic polynomial at k
    pub fn chromatic_number_at(&self, k: i64) -> i64 {
        let poly = self.chromatic_polynomial();
        let mut result = 0i64;
        let mut k_power = 1i64;
        for &coeff in &poly {
            result += coeff * k_power;
            k_power *= k;
        }
        result
    }

    /// Greedy coloring: returns (number of colors used, color assignments)
    pub fn greedy_coloring(&self) -> (usize, Vec<usize>) {
        let mut colors = vec![None; self.n];
        for u in 0..self.n {
            let mut used = vec![false; self.n + 1];
            for &v in &self.adj[u] {
                if let Some(c) = colors[v] {
                    used[c] = true;
                }
            }
            let color = (0..=self.n).find(|&c| !used[c]).unwrap();
            colors[u] = Some(color);
        }
        let color_vec: Vec<usize> = colors.iter().map(|c| c.unwrap()).collect();
        let num_colors = *color_vec.iter().max().unwrap_or(&0) + 1;
        (num_colors, color_vec)
    }

    /// Create a complete graph K_n
    pub fn complete(n: usize) -> Graph {
        let mut g = Graph::new(n);
        for i in 0..n {
            for j in (i + 1)..n {
                g.add_edge(i, j);
            }
        }
        g
    }

    /// Create a cycle graph C_n
    pub fn cycle(n: usize) -> Graph {
        let mut g = Graph::new(n);
        for i in 0..n {
            g.add_edge(i, (i + 1) % n);
        }
        g
    }

    /// Create a path graph P_n
    pub fn path(n: usize) -> Graph {
        let mut g = Graph::new(n);
        for i in 0..n.saturating_sub(1) {
            g.add_edge(i, i + 1);
        }
        g
    }
}

fn poly_add(a: &[i64], b: &[i64]) -> Vec<i64> {
    let len = a.len().max(b.len());
    let mut result = vec![0i64; len];
    for i in 0..a.len() { result[i] += a[i]; }
    for i in 0..b.len() { result[i] += b[i]; }
    result
}

fn poly_sub(a: &[i64], b: &[i64]) -> Vec<i64> {
    let len = a.len().max(b.len());
    let mut result = vec![0i64; len];
    for i in 0..a.len() { result[i] += a[i]; }
    for i in 0..b.len() { result[i] -= b[i]; }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chromatic_complete_graph() {
        // P(K_n, k) = k(k-1)(k-2)...(k-n+1)
        let g = Graph::complete(3);
        // k(k-1)(k-2) = k^3 - 3k^2 + 2k
        assert_eq!(g.chromatic_number_at(3), 6); // 3*2*1
        assert_eq!(g.chromatic_number_at(4), 24); // 4*3*2
        assert_eq!(g.chromatic_number_at(2), 0);
    }

    #[test]
    fn test_chromatic_path() {
        // P(P_n, k) = k(k-1)^{n-1}
        let g = Graph::path(3);
        // k(k-1)^2 = k^3 - 2k^2 + k
        assert_eq!(g.chromatic_number_at(3), 12); // 3*2*2
        assert_eq!(g.chromatic_number_at(2), 2); // 2*1*1
    }

    #[test]
    fn test_chromatic_cycle() {
        // P(C_n, k) = (k-1)^n + (-1)^n * (k-1)
        let g = Graph::cycle(3);
        // P(C_3, k) = (k-1)^3 + (k-1) = k^3 - 3k^2 + 3k - 1 + k - 1 = k^3 - 3k^2 + 4k - 2
        // Wait: P(C_3) = (k-1)^3 + (k-1) for n=3 (odd, so + (k-1))
        // k=3: 8 + 2 = 10... no, C_3 = K_3, should be 6
        // Actually: P(C_n, k) = (k-1)^n + (-1)^n*(k-1)
        // For C_3: (k-1)^3 + (-1)^3*(k-1) = (k-1)^3 - (k-1)
        // k=3: 8 - 2 = 6 ✓
        assert_eq!(g.chromatic_number_at(3), 6);
        assert_eq!(g.chromatic_number_at(2), 0); // odd cycle needs 3 colors
    }

    #[test]
    fn test_chromatic_cycle_4() {
        let g = Graph::cycle(4);
        // P(C_4, k) = (k-1)^4 + (k-1) = k^4 - 4k^3 + 6k^2 - 3k
        // k=2: 1+1 = 2
        assert_eq!(g.chromatic_number_at(2), 2);
        assert_eq!(g.chromatic_number_at(3), 18);
    }

    #[test]
    fn test_greedy_coloring_path() {
        let g = Graph::path(4);
        let (colors, _) = g.greedy_coloring();
        assert_eq!(colors, 2); // Path is bipartite
    }

    #[test]
    fn test_greedy_coloring_complete() {
        let g = Graph::complete(4);
        let (colors, _) = g.greedy_coloring();
        assert_eq!(colors, 4);
    }
}
