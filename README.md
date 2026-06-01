# lau-combinatorics

Combinatorics library for Rust — counting structures, graph theory, enumeration, generating functions, Burnside's lemma, Catalan numbers, integer partitions, and agent topology analysis.

## What This Does

This crate covers the major pillars of enumerative and algebraic combinatorics:

| Module | Purpose |
|---|---|
| `basic` | Factorials, binomial/multinomial coefficients, permutations, Stirling numbers, Bell numbers, stars and bars |
| `catalan` | Catalan numbers, Dyck paths, triangulations, ballot problem, Narayana numbers |
| `partition` | Integer partitions, Young diagram conjugation, Euler's pentagonal theorem, restricted/distinct partitions |
| `generating` | Ordinary and exponential generating functions (OGF/EGF) with Cauchy and binomial convolution |
| `inclusion_exclusion` | Inclusion-exclusion principle, Euler's totient, derangements, coprime counting |
| `pigeonhole` | Pigeonhole principle applications, Erdős–Szekeres bound, LIS (patience sorting), birthday problem |
| `burnside` | Burnside's lemma, necklace/cyclic colorings, cube vertex colorings, permutation-group colorings |
| `graph_coloring` | Chromatic polynomial (deletion-contraction), greedy coloring, standard graph constructors |
| `tree_enum` | Cayley's formula, Prüfer sequences (encode/decode), spanning trees (bipartite formula, Kirchhoff's theorem) |
| `agent_topology` | Agent configuration counting: topologies, teams, hierarchies, pipelines, fault-tolerance |

**71 tests** across all modules.

## Key Idea

Combinatorics is about counting without enumerating. This crate gives you the formulas and algorithms to count combinatorial objects directly — from simple binomial coefficients to chromatic polynomials — along with the structural tools (generating functions, Prüfer sequences, partition enumeration) to work with the objects themselves.

## Install

```toml
[dependencies]
lau-combinatorics = "0.1.0"
```

Rust 2021 edition. Dependencies:
- `nalgebra` 0.35 — for Kirchhoff's matrix tree theorem (determinant)
- `serde` 1 (with `derive`) — serialization

## Quick Start

### Binomial coefficients and factorials

```rust
use lau_combinatorics::*;

assert_eq!(factorial(10), 3628800);
assert_eq!(binomial(10, 3), 120);
assert_eq!(permutations(5, 3), 60);
assert_eq!(multinomial(6, &[2, 2, 2]), 90);
```

### Catalan numbers and structures

```rust
use lau_combinatorics::*;

// Catalan numbers: 1, 1, 2, 5, 14, 42, 132, ...
assert_eq!(catalan(5), 42);

// Each Catalan number counts many structures:
assert_eq!(dyck_paths(4), 14);           // lattice paths
assert_eq!(triangulations(4), 14);       // polygon triangulations
assert_eq!(valid_parentheses(4), 14);    // balanced parens
assert_eq!(binary_search_trees(4), 14);  // BST shapes

// Narayana numbers: Dyck paths with k peaks
assert_eq!(narayana(4, 2), 6);
```

### Integer partitions

```rust
use lau_combinatorics::*;

// Number of partitions
assert_eq!(partition_number(10), 42);
assert_eq!(partition_number(20), 627);

// Euler's pentagonal theorem gives the same results
assert_eq!(partition_pentagonal(20), 627);

// Enumerate all partitions of 5
let parts = partitions(5);
// [[5], [4,1], [3,2], [3,1,1], [2,2,1], [2,1,1,1], [1,1,1,1,1]]
assert_eq!(parts.len(), 7);

// Young diagram conjugation: [5,3,1] → [3,2,2,1,1]
assert_eq!(conjugate_partition(&[5, 3, 1]), vec![3, 2, 2, 1, 1]);
```

### Generating functions

```rust
use lau_combinatorics::*;

// OGF for Fibonacci
let fib = OGF::fibonacci(10);
assert_eq!(fib.coeff(7), 13);

// OGF multiplication: (1+x)^2 = 1 + 2x + x^2
let a = OGF::new(vec![1, 1]);
let c = a.multiply(&a);
assert_eq!(c.coeffs, vec![1, 2, 1]);

// EGF: exp(x) * exp(x) = exp(2x), coefficients are 2^n
let e = EGF::exponential(5);
let e2 = e.multiply(&e);
assert_eq!(e2.raw_coeff(4), 16);
```

### Inclusion-exclusion and number theory

```rust
use lau_combinatorics::*;

// Derangements: permutations with no fixed points
assert_eq!(derangements(5), 44);

// Euler's totient
assert_eq!(euler_totient(30), 8);

// Count integers in [1, n] not divisible by given primes
assert_eq!(count_coprimes(30, &[2, 3, 5]), 8);
```

### Burnside's lemma and necklace colorings

```rust
use lau_combinatorics::*;

// Distinct 2-color necklaces with 4 beads = 6
assert_eq!(necklace_colorings(4, 2), 6);

// Cube vertex colorings with 2 colors = 23
assert_eq!(cube_vertex_colorings(2), 23);
```

### Graph coloring

```rust
use lau_combinatorics::*;

// Chromatic polynomial of K_3: k(k-1)(k-2)
let g = Graph::complete(3);
assert_eq!(g.chromatic_number_at(3), 6);

// Greedy coloring of a path
let path = Graph::path(4);
let (colors, assignment) = path.greedy_coloring();
assert_eq!(colors, 2);
```

### Tree enumeration and Prüfer sequences

```rust
use lau_combinatorics::*;

// Cayley's formula: n^{n-2} labeled trees
assert_eq!(cayley(4), 16);

// Convert tree ↔ Prüfer sequence
let edges = vec![(0, 1), (1, 2), (2, 3)];
let prufer = tree_to_prufer(&edges);
let recovered = prufer_to_tree(&prufer);

// Spanning trees via Kirchhoff's theorem
let k4_edges: Vec<_> = (0..4).flat_map(|i| ((i+1)..4).map(move |j| (i,j))).collect();
assert_eq!(spanning_trees_matrix(&k4_edges, 4), 16);
```

### Agent topology counting

```rust
use lau_combinatorics::*;

let summary = topology_summary(3);
// 3 agents: 3 comm topologies, 5 team partitions, 9 hierarchies, 6 pipeline configs
assert_eq!(summary.communication_topologies, 3);
assert_eq!(summary.all_team_partitions, 5);
assert_eq!(summary.hierarchies, 9);
```

## API Reference

### Basic Counting (`basic`)

| Function | Formula |
|---|---|
| `factorial(n)` | n! |
| `permutations(n, k)` | P(n, k) = n!/(n−k)! |
| `binomial(n, k)` | C(n, k) = n!/(k!(n−k)!) |
| `multinomial(n, ks)` | n!/(k₁!·k₂!·…·kₘ!) |
| `falling_factorial(n, k)` | n·(n−1)·…·(n−k+1) |
| `rising_factorial(n, k)` | n·(n+1)·…·(n+k−1) |
| `stars_and_bars(n, k)` | C(n+k−1, k−1) |
| `stirling_second(n, k)` | S(n, k) — partitions into k non-empty subsets |
| `bell_number(n)` | B(n) — total partitions of n-element set |

### Catalan Structures (`catalan`)

All return Cₙ = (2n)!/((n+1)!·n!):

`catalan(n)`, `dyck_paths(n)`, `full_binary_trees(n)`, `valid_parentheses(n)`, `non_crossing_partitions(n)`, `triangulations(n)`, `binary_search_trees(n)`.

Also: `ballot_number(a, b)`, `narayana(n, k)`, `catalan_sequence(n)`.

### Partitions (`partition`)

| Function | Description |
|---|---|
| `partition_number(n)` | p(n) via DP |
| `partition_pentagonal(n)` | p(n) via Euler's pentagonal theorem |
| `partitions(n)` | Enumerate all partitions as vectors of parts |
| `conjugate_partition(p)` | Young diagram transpose |
| `restricted_partitions(n, k)` | Partitions into at most k parts |
| `distinct_partitions(n)` | Partitions with all parts distinct |

### Generating Functions (`generating`)

**OGF** (Ordinary): `new(coeffs)`, `multiply()`, `coeff(n)`, `truncate(n)`, `negative_binomial(k, terms)`, `geometric(terms)`, `fibonacci(terms)`.

**EGF** (Exponential): `new(coeffs)`, `multiply()` (binomial convolution), `raw_coeff(n)`, `exponential(terms)`, `bell(terms)`.

### Inclusion-Exclusion (`inclusion_exclusion`)

| Function | Description |
|---|---|
| `inclusion_exclusion(sizes)` | Union size from bitmask-indexed intersection sizes |
| `count_coprimes(n, primes)` | Count [1..n] not divisible by any given prime |
| `derangements(n)` | !n — permutations with no fixed points |
| `euler_totient(n)` | φ(n) — coprime count |

### Pigeonhole (`pigeonhole`)

| Function | Description |
|---|---|
| `min_max_load(n, k)` | ⌈n/k⌉ — minimum guaranteed max bin load |
| `erdos_szekeres_bound(n)` | ⌈√n⌉ — monotone subsequence guarantee |
| `longest_increasing_subsequence(seq)` | LIS length via patience sorting |
| `find_duplicate(nums)` | Find duplicate in n+1 values from [1..n] |
| `birthday_collision_threshold(d, threshold)` | Smallest n for collision probability ≥ threshold |

### Burnside (`burnside`)

| Function | Description |
|---|---|
| `burnside(group_size, fix_counts)` | Orbit count via fix-point averaging |
| `necklace_colorings(n, k)` | Distinct k-color necklaces (dihedral group) |
| `cyclic_colorings(n, k)` | Distinct k-color necklaces (cyclic group only) |
| `cube_vertex_colorings(k)` | Distinct k-colorings of cube vertices |
| `count_colorings_under_group(perms, k)` | General permutation-group coloring count |

### Graph Coloring (`graph_coloring`)

`Graph` struct with adjacency list.

| Method | Description |
|---|---|
| `new(n)` / `add_edge(u, v)` | Construct graph |
| `chromatic_polynomial()` | Deletion-contraction → polynomial coefficients |
| `chromatic_number_at(k)` | Evaluate chromatic polynomial at k |
| `greedy_coloring()` | Returns (num_colors, color_assignments) |
| `complete(n)` / `cycle(n)` / `path(n)` | Standard graph constructors |

### Tree Enumeration (`tree_enum`)

| Function | Description |
|---|---|
| `cayley(n)` | n^{n−2} labeled trees |
| `tree_to_prufer(edges)` | Encode tree → Prüfer sequence |
| `prufer_to_tree(prufer)` | Decode Prüfer sequence → edge list |
| `spanning_trees_bipartite(m, n)` | Spanning trees of K_{m,n} |
| `spanning_trees_matrix(edges, n)` | Kirchhoff's matrix tree theorem |

### Agent Topology (`agent_topology`)

| Function | Formula |
|---|---|
| `count_communication_topologies(n)` | n^{n−2} (Cayley) |
| `count_team_assignments(n, k)` | S(n, k) (Stirling) |
| `count_all_team_assignments(n)` | B(n) (Bell) |
| `count_task_distributions(n, k)` | C(n+k−1, k−1) (stars & bars) |
| `count_distinct_task_assignments(n, k)` | k^n |
| `count_hierarchies(n)` | n^{n−1} |
| `count_pipeline_configs(n)` | n! |
| `count_role_assignments(n, r)` | r^n |
| `count_fault_tolerant_configs(n, k)` | C(n, k) |
| `topology_summary(n)` | `TopologySummary` with all counts |

## How It Works

### Binomial Coefficients

Uses the multiplicative formula to avoid overflow in intermediate factorials:

$$\binom{n}{k} = \prod_{i=0}^{k-1} \frac{n-i}{i+1}$$

Division at each step is exact because the partial product is always divisible by `i+1`.

### Stirling Numbers

Computed via dynamic programming using the recurrence:

$$S(n, k) = k \cdot S(n-1, k) + S(n-1, k-1)$$

### Euler's Pentagonal Theorem

Computes p(n) using the recurrence:

$$p(n) = \sum_{k \neq 0} (-1)^{k+1} \cdot p\!\left(n - \frac{k(3k-1)}{2}\right)$$

where the sum runs over generalized pentagonal numbers. This is much faster than the basic DP for large n.

### Generating Functions

- **OGF multiplication**: Cauchy product — c_n = Σ aᵢ · b_{n−i}
- **EGF multiplication**: Binomial convolution — c_n = Σ C(n,k) · aₖ · b_{n−k}

Pre-built generators for geometric series, Fibonacci, negative binomial, Bell numbers, and exp(x).

### Chromatic Polynomial

Computed via the deletion-contraction recurrence:

$$P(G, k) = P(G - e, k) - P(G / e, k)$$

where G−e is the graph with edge e removed, and G/e is the graph with edge e contracted. Base case: edgeless graph on n vertices → kⁿ.

### Prüfer Sequences

Encode: repeatedly remove the smallest leaf and record its neighbor. The sequence of neighbors is the Prüfer code (length n−2).

Decode: maintain degree counts, repeatedly connect the smallest leaf to the next Prüfer element.

### Kirchhoff's Matrix Tree Theorem

Build the Laplacian matrix L of the graph (degree diagonal, −1 for edges), delete any row and column, take the determinant. The result equals the number of spanning trees.

## The Math

### Catalan Numbers

$$C_n = \frac{1}{n+1}\binom{2n}{n} = \frac{(2n)!}{(n+1)!\,n!}$$

Catalan numbers count an enormous variety of combinatorial structures. This crate provides direct functions for the most common interpretations: Dyck paths, binary trees, BSTs, valid parentheses, non-crossing partitions, and polygon triangulations.

The **Narayana numbers** refine Catalan: N(n, k) counts Dyck paths of semilength n with exactly k peaks:

$$N(n, k) = \frac{1}{n}\binom{n}{k}\binom{n}{k-1}$$

### Burnside's Lemma

For a finite group G acting on a set X:

$$|X/G| = \frac{1}{|G|} \sum_{g \in G} |X^g|$$

where X^g is the set of elements fixed by g. For necklace colorings, the group is the dihedral group D_n (rotations + reflections). Rotations by r contribute k^{gcd(n,r)} fixed colorings.

### Inclusion-Exclusion

$$|A_1 \cup A_2 \cup \cdots \cup A_n| = \sum_{\emptyset \neq S \subseteq [n]} (-1)^{|S|+1} \left|\bigcap_{i \in S} A_i\right|$$

Applications include counting coprimes (sieve), derangements (D(n) = n! · Σ(−1)ᵏ/k!), and Euler's totient (φ(n) = n · Π(1 − 1/p) over prime factors).

### Pigeonhole Principle

If n items go into k bins, at least one bin has ⌈n/k⌉ items. The crate applies this to:
- **Erdős–Szekeres**: any sequence of n²+1 distinct reals has a monotone subsequence of length n+1
- **Birthday problem**: with d possible values, only ≈√(2d ln 2) samples needed for 50% collision chance

## License

MIT
