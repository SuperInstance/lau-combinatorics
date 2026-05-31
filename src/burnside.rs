//! Burnside's lemma for counting under symmetry


/// Burnside's lemma: given a group action, count the number of distinct orbits.
/// |X/G| = (1/|G|) * sum_{g in G} |Fix(g)|
///
/// `fix_counts` is a vector where fix_counts[i] = |Fix(g_i)| for each group element g_i.
pub fn burnside(group_size: u64, fix_counts: &[u64]) -> u64 {
    let sum: u64 = fix_counts.iter().sum();
    assert!(sum % group_size == 0, "Burnside result must be integer");
    sum / group_size
}

/// Count distinct colorings of n beads on a necklace with k colors,
/// where rotations and reflections are considered equivalent (dihedral group D_n).
pub fn necklace_colorings(n: u64, k: u64) -> u64 {
    if n == 0 { return 1; }
    // Dihedral group D_n has 2n elements: n rotations + n reflections
    let mut fix_counts = Vec::new();

    // Rotations by r positions: fixes = k^{gcd(n, r)}
    for r in 0..n {
        fix_counts.push(k.pow(gcd(n, r) as u32));
    }

    // Reflections
    if n % 2 == 0 {
        // n/2 reflections through opposite vertices: k^{n/2 + 1}
        // n/2 reflections through opposite edges: k^{n/2}
        for _ in 0..n / 2 {
            fix_counts.push(k.pow((n / 2 + 1) as u32));
        }
        for _ in 0..n / 2 {
            fix_counts.push(k.pow((n / 2) as u32));
        }
    } else {
        // n reflections through a vertex and opposite edge: k^{(n+1)/2}
        for _ in 0..n {
            fix_counts.push(k.pow(((n + 1) / 2) as u32));
        }
    }

    burnside(2 * n, &fix_counts)
}

/// Count distinct colorings under cyclic group (rotations only, no reflections).
pub fn cyclic_colorings(n: u64, k: u64) -> u64 {
    if n == 0 { return 1; }
    let fix_counts: Vec<u64> = (0..n).map(|r| k.pow(gcd(n, r) as u32)).collect();
    burnside(n, &fix_counts)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Count distinct colorings of vertices of a cube with k colors under rotational symmetry.
/// The rotation group of a cube has 24 elements.
pub fn cube_vertex_colorings(k: u64) -> u64 {
    let k2 = k * k;
    let k3 = k2 * k;
    // 1 identity: k^8
    // 6 face rotations (90/270): k^2 each
    // 3 face rotations (180): k^4 each
    // 8 vertex rotations (120/240): k^4 each  -- actually k^4? No.
    // Vertex rotations cycle through 4 sets of 2 vertices, so k^4... 
    // Wait: cube has 8 vertices. A vertex rotation (120°) cycles 3 groups of 4? No.
    // Let me be precise:
    // Identity (1): k^8
    // 90°/270° face rotations (6): each fixes k^2 vertices (the two face centers' opposite vertices cycle)
    //   Actually for vertices: a face rotation by 90° partitions 8 vertices into:
    //   4 on top face cycle, 4 on bottom face cycle → k^2
    // 180° face rotations (3): top 4 split into 2 pairs, bottom 4 into 2 pairs → k^4
    // 120°/240° vertex rotations (8): the 8 vertices cycle as: 2 fixed + 2 orbits of 3? No.
    //   Actually a vertex rotation fixes 2 opposite vertices and cycles the other 6 in 2 orbits of 3.
    //   But wait, there are no fixed vertices in a proper rotation. Let me reconsider.
    //   A 120° rotation about a body diagonal: the 2 vertices on the diagonal are fixed? No, they're swapped? No, they stay.
    //   So: 2 fixed + 2 cycles of 3 → k^{2+2} = k^4... no, fixed points contribute k each, cycles contribute k each.
    //   2 fixed vertices: each can be any color → k^2
    //   2 cycles of 3 vertices: each cycle must be monochrome → k^2
    //   Total: k^4
    // 180° edge rotations (6): cycle 4 pairs of vertices → k^4
    let fix_counts = vec![
        k3 * k3 * k * k, // k^8: identity (1 element)
        k2, k2, k2, k2, k2, k2, // 90°/270° face: 6 elements, k^2 each
        k3 * k, k3 * k, k3 * k, // 180° face: 3 elements, k^4 each
        k3 * k, k3 * k, k3 * k, k3 * k, // 120°/240° vertex: 8 elements, k^4 each
        k3 * k, k3 * k, k3 * k, k3 * k, // continuing
        k3 * k, k3 * k, // 180° edge: 6 elements, k^4 each
        k3 * k, k3 * k, k3 * k, k3 * k,
    ];
    assert_eq!(fix_counts.len(), 24);
    burnside(24, &fix_counts)
}

/// Count distinct colorings under a given permutation group.
/// `permutations` is a list of permutations, each as a mapping from position to position.
/// `k` is the number of colors.
pub fn count_colorings_under_group(permutations: &[Vec<usize>], k: u64) -> u64 {
    let fix_counts: Vec<u64> = permutations.iter().map(|perm| {
        // Count cycles in the permutation
        let n = perm.len();
        let mut visited = vec![false; n];
        let mut cycles = 0;
        for i in 0..n {
            if !visited[i] {
                cycles += 1;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    j = perm[j];
                }
            }
        }
        k.pow(cycles as u32)
    }).collect();
    burnside(permutations.len() as u64, &fix_counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_burnside_basic() {
        // Simple case: 2 group elements, both fix everything
        assert_eq!(burnside(2, &[4, 4]), 4);
    }

    #[test]
    fn test_necklace_2_colors_4_beads() {
        // 2 colors, 4 beads: should be 6 distinct necklaces
        // (with rotations + reflections)
        assert_eq!(necklace_colorings(4, 2), 6);
    }

    #[test]
    fn test_necklace_3_colors_3_beads() {
        assert_eq!(necklace_colorings(3, 3), 10);
    }

    #[test]
    fn test_cyclic_colorings() {
        // Cyclic colorings of 4 beads with 2 colors (rotations only)
        assert_eq!(cyclic_colorings(4, 2), 6);
    }

    #[test]
    fn test_cyclic_colorings_2_2() {
        // 2 beads, 2 colors, rotations only: RR, RB, BB → 3
        assert_eq!(cyclic_colorings(2, 2), 3);
    }

    #[test]
    fn test_count_colorings_under_group() {
        // S_3 acting on 3 elements with 2 colors
        // Permutations of {0,1,2}
        let perms = vec![
            vec![0, 1, 2], // identity
            vec![1, 0, 2], // (01)
            vec![0, 2, 1], // (12)
            vec![2, 1, 0], // (02)
            vec![1, 2, 0], // (012)
            vec![2, 0, 1], // (021)
        ];
        // Burnside: (2^3 + 2^2 + 2^2 + 2^2 + 2^1 + 2^1) / 6 = (8+4+4+4+2+2)/6 = 24/6 = 4
        assert_eq!(count_colorings_under_group(&perms, 2), 4);
    }

    #[test]
    fn test_cube_vertex_colorings() {
        // Cube with 1 color: 1
        assert_eq!(cube_vertex_colorings(1), 1);
        // Cube with 2 colors: 23
        assert_eq!(cube_vertex_colorings(2), 23);
    }
}
