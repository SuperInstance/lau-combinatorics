# lau-combinatorics

> Combinatorics library — counting structures, graph theory, and enumeration

## What This Does

Combinatorics library — counting structures, graph theory, and enumeration. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-combinatorics
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_combinatorics::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub fn inclusion_exclusion(intersection_sizes: &[u64]) -> u64 
pub fn count_coprimes(n: u64, primes: &[u64]) -> u64 
pub fn derangements(n: u64) -> u64 
pub fn euler_totient(mut n: u64) -> u64 
pub struct Graph 
    pub fn new(n: usize) -> Self 
    pub fn add_edge(&mut self, u: usize, v: usize) 
    pub fn chromatic_polynomial(&self) -> Vec<i64> 
    pub fn chromatic_number_at(&self, k: i64) -> i64 
    pub fn greedy_coloring(&self) -> (usize, Vec<usize>) 
    pub fn complete(n: usize) -> Graph 
    pub fn cycle(n: usize) -> Graph 
    pub fn path(n: usize) -> Graph 
pub fn factorial(n: u64) -> u64 
pub fn permutations(n: u64, k: u64) -> u64 
pub fn combinations(n: u64, k: u64) -> u64 
pub fn binomial(n: u64, k: u64) -> u64 
pub fn multinomial(n: u64, ks: &[u64]) -> u64 
pub fn falling_factorial(n: u64, k: u64) -> u64 
pub fn rising_factorial(n: u64, k: u64) -> u64 
pub fn stars_and_bars(n: u64, k: u64) -> u64 
pub fn stirling_second(n: u64, k: u64) -> u64 
pub fn bell_number(n: u64) -> u64 
pub fn partition_number(n: usize) -> u64 
pub fn partition_numbers(n: usize) -> Vec<u64> 
pub fn partition_pentagonal(n: usize) -> u64 
pub fn partitions(n: u64) -> Vec<Vec<u64>> 
pub fn conjugate_partition(partition: &[u64]) -> Vec<u64> 
pub fn restricted_partitions(n: u64, k: u64) -> u64 
pub fn distinct_partitions(n: u64) -> Vec<Vec<u64>> 
pub fn cayley(n: u64) -> u64 
pub fn tree_to_prufer(edges: &[(usize, usize)]) -> Vec<usize> 
pub fn prufer_to_tree(prufer: &[usize]) -> Vec<(usize, usize)> 
pub fn spanning_trees_bipartite(m: u64, n: u64) -> u64 
pub fn spanning_trees_matrix(edges: &[(usize, usize)], n: usize) -> u64 
pub fn min_max_load(n: u64, k: u64) -> u64 
pub fn erdos_szekeres_bound(n: u64) -> u64 
pub fn longest_increasing_subsequence(seq: &[i64]) -> usize 
pub fn find_duplicate(nums: &[u64]) -> Option<u64> 
pub fn birthday_collision_threshold(d: u64, threshold: f64) -> u64 
pub struct OGF 
    pub fn new(coeffs: Vec<u64>) -> Self 
    pub fn multiply(&self, other: &OGF) -> OGF 
    pub fn coeff(&self, n: usize) -> u64 
    pub fn truncate(&self, n: usize) -> OGF 
    pub fn negative_binomial(k: u64, terms: usize) -> OGF 
    pub fn geometric(terms: usize) -> OGF 
    pub fn fibonacci(terms: usize) -> OGF 
pub struct EGF 
    pub fn new(coeffs: Vec<u64>) -> Self 
    pub fn multiply(&self, other: &EGF) -> EGF 
    pub fn raw_coeff(&self, n: usize) -> u64 
    pub fn exponential(terms: usize) -> EGF 
    pub fn bell(terms: usize) -> EGF 
pub fn count_communication_topologies(n: u64) -> u64 
pub fn count_team_assignments(n: u64, k: u64) -> u64 
pub fn count_all_team_assignments(n: u64) -> u64 
pub fn count_task_distributions(n: u64, k: u64) -> u64 
pub fn count_distinct_task_assignments(n: u64, k: u64) -> u64 
pub fn count_hierarchies(n: u64) -> u64 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**66 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
