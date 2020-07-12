pub fn map<T, O, F>(input: Vec<T>, f: F) -> Vec<O>
where
    F: FnMut(T) -> O,
{
    input.into_iter().map(f).collect()
}
