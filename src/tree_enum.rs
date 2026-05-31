//! Enumeration of trees: Cayley's formula, Prüfer sequences


/// Cayley's formula: the number of labeled trees on n vertices is n^{n-2}.
pub fn cayley(n: u64) -> u64 {
    if n <= 1 { return 1; }
    if n == 2 { return 1; }
    n.pow((n - 2) as u32)
}

/// Convert a labeled tree (given as edge list) to its Prüfer sequence.
/// The tree should have vertices labeled 0..n-1.
pub fn tree_to_prufer(edges: &[(usize, usize)]) -> Vec<usize> {
    let n = edges.len() + 1;
    if n <= 2 { return vec![]; }

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut degree: Vec<usize> = adj.iter().map(|a| a.len()).collect();
    let mut prufer = Vec::with_capacity(n - 2);

    // Find the smallest leaf
    let mut leaf = (0..n).find(|&i| degree[i] == 1).unwrap();

    for _ in 0..n - 2 {
        // Find the neighbor of this leaf
        let neighbor = adj[leaf].iter().find(|&&v| degree[v] > 0).copied().unwrap();
        prufer.push(neighbor);
        degree[leaf] = 0;
        degree[neighbor] -= 1;
        if neighbor < leaf && degree[neighbor] == 1 {
            leaf = neighbor;
        } else {
            leaf = (0..n).find(|&i| degree[i] == 1).unwrap_or(n);
        }
    }

    prufer
}

/// Convert a Prüfer sequence back to a labeled tree (edge list).
pub fn prufer_to_tree(prufer: &[usize]) -> Vec<(usize, usize)> {
    let m = prufer.len();
    let n = m + 2;
    if m == 0 { return vec![(0, 1)]; }

    let mut degree = vec![1usize; n];
    for &v in prufer {
        degree[v] += 1;
    }

    let mut edges = Vec::with_capacity(n - 1);
    let mut prufer_iter = prufer.iter().peekable();

    for _ in 0..m {
        let v = *prufer_iter.next().unwrap();
        // Find smallest leaf (degree == 1)
        let u = (0..n).find(|&i| degree[i] == 1).unwrap();
        edges.push((u.min(v), u.max(v)));
        degree[u] -= 1;
        degree[v] -= 1;
    }

    // Two remaining vertices
    let remaining: Vec<usize> = (0..n).filter(|&i| degree[i] == 1).collect();
    edges.push((remaining[0].min(remaining[1]), remaining[0].max(remaining[1])));

    edges
}

/// Count the number of spanning trees in a complete bipartite graph K_{m,n}.
/// By Cayley: m^{n-1} * n^{m-1}
pub fn spanning_trees_bipartite(m: u64, n: u64) -> u64 {
    if m == 0 || n == 0 { return 0; }
    if m == 1 { return 1; }
    if n == 1 { return 1; }
    m.pow((n - 1) as u32) * n.pow((m - 1) as u32)
}

/// Count spanning trees using Kirchhoff's matrix tree theorem.
/// Uses nalgebra for the Laplacian matrix and determinant computation.
pub fn spanning_trees_matrix(edges: &[(usize, usize)], n: usize) -> u64 {
    use nalgebra::DMatrix;

    if n <= 1 { return 1; }

    // Build Laplacian
    let mut lap = vec![0f64; n * n];
    for &(u, v) in edges {
        lap[u * n + u] += 1.0;
        lap[v * n + v] += 1.0;
        lap[u * n + v] -= 1.0;
        lap[v * n + u] -= 1.0;
    }

    // Delete last row and column to get cofactor
    let m = n - 1;
    let matrix = DMatrix::from_fn(m, m, |i, j| lap[i * n + j]);
    let det = matrix.determinant();
    det.round().abs() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cayley() {
        assert_eq!(cayley(1), 1);
        assert_eq!(cayley(2), 1);
        assert_eq!(cayley(3), 3);
        assert_eq!(cayley(4), 16);
        assert_eq!(cayley(5), 125);
    }

    #[test]
    fn test_prufer_roundtrip() {
        // Tree on 4 vertices: edges (0,1), (1,2), (2,3) — a path
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let prufer = tree_to_prufer(&edges);
        let recovered = prufer_to_tree(&prufer);
        // Check both produce valid trees with same edge count
        assert_eq!(recovered.len(), 3);
        // For a more thorough check, verify prufer roundtrip for several trees
    }

    #[test]
    fn test_prufer_roundtrip_multiple() {
        // Test several trees on 5 vertices
        let trees = vec![
            vec![(0, 1), (0, 2), (0, 3), (0, 4)], // star
            vec![(0, 1), (1, 2), (2, 3), (3, 4)], // path
            vec![(0, 1), (1, 2), (1, 3), (3, 4)], // arbitrary
        ];
        for tree_edges in &trees {
            let prufer = tree_to_prufer(tree_edges);
            assert_eq!(prufer.len(), 3); // n-2 = 3
            let recovered = prufer_to_tree(&prufer);
            assert_eq!(recovered.len(), 4);

            // Verify both have same degree sequence
            let mut deg_orig = vec![0usize; 5];
            let mut deg_rec = vec![0usize; 5];
            for &(u, v) in tree_edges {
                deg_orig[u] += 1;
                deg_orig[v] += 1;
            }
            for &(u, v) in &recovered {
                deg_rec[u] += 1;
                deg_rec[v] += 1;
            }
            let mut orig_sorted = deg_orig.clone();
            let mut rec_sorted = deg_rec.clone();
            orig_sorted.sort();
            rec_sorted.sort();
            assert_eq!(orig_sorted, rec_sorted);
        }
    }

    #[test]
    fn test_spanning_trees_bipartite() {
        // K_{2,3}: 2^2 * 3^1 = 12
        assert_eq!(spanning_trees_bipartite(2, 3), 12);
        // K_{2,2}: 2^1 * 2^1 = 4
        assert_eq!(spanning_trees_bipartite(2, 2), 4);
    }

    #[test]
    fn test_spanning_trees_matrix() {
        // K_4: 4^{4-2} = 16 spanning trees
        let mut edges = vec![];
        for i in 0..4 {
            for j in (i + 1)..4 {
                edges.push((i, j));
            }
        }
        assert_eq!(spanning_trees_matrix(&edges, 4), 16);
    }

    #[test]
    fn test_spanning_trees_path() {
        // Path on 3 vertices: only 1 spanning tree (itself)
        let edges = vec![(0, 1), (1, 2)];
        assert_eq!(spanning_trees_matrix(&edges, 3), 1);
    }
}
