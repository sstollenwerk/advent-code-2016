pub fn all_eq<T: Eq>(xs: &[T]) -> bool {
    let first = xs.get(0);
    xs.iter().all(|i| Some(i) == first)
}
