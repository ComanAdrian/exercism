pub fn find<T, K>(array: T, key: K) -> Option<usize>
where
    T: AsRef<[K]>,
    K: PartialOrd,
{
    let total_iterations = f64::from(array.as_ref().len() as i32).log2().ceil() as i32;

    let mut diff = array.as_ref().len() / 2;
    let mut index = diff;

    for _ in 0..=total_iterations {
        diff = if diff > 1 { diff / 2 } else { 1 };

        if *array.as_ref().get(index).unwrap() > key {
            if index > 0 {
                index -= diff;
            }
        } else if *array.as_ref().get(index).unwrap() < key {
            index += diff;
        } else {
            return Some(index);
        }
    }

    return None;
}
