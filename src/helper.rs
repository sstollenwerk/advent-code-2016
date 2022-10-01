pub fn all_eq<T: Eq>(xs: &[T]) -> bool {
    let first = xs.get(0);
    xs.iter().all(|i| Some(i) == first)
}

pub fn as_u4(vals: &[u8]) -> Vec<u8> {
    vals.iter().flat_map(|k| [k / 16, k % 16]).collect()
}
