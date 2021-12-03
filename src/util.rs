pub fn unradix(rev_digits: impl Iterator<Item = usize>, radix: usize) -> usize {
    rev_digits
        .zip(itertools::iterate(1, |i| radix * i))
        .map(|p| p.0 * p.1)
        .sum()
}
