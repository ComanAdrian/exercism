use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T>
where
    T: Eq + Hash + Copy + Debug,
{
    inner: HashSet<T>,
}

impl<T> CustomSet<T>
where
    T: Eq + Hash + Copy + Debug,
{
    pub fn new(input: &[T]) -> Self {
        let mut hash_set: HashSet<T> = HashSet::new();
        input.into_iter().for_each(|&v| {
            // move occurs because `*v` has type `T`,
            // which does not implement the `Copy` trait
            hash_set.insert(v);
        });
        CustomSet { inner: hash_set }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.inner.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.inner.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.inner.is_subset(&other.inner)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.inner.is_disjoint(&other.inner)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        self.inner.intersection(&other.inner).map(|&v| v).collect()
    }

    pub fn difference(&self, other: &Self) -> Self {
        self.inner.difference(&other.inner).map(|&v| v).collect()
    }

    pub fn union(&self, other: &Self) -> Self {
        self.inner.union(&other.inner).map(|&v| v).collect()
    }
}

impl<T> FromIterator<T> for CustomSet<T>
where
    T: Eq + Hash + Copy + Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = CustomSet::new(&[]);

        for i in iter {
            c.add(i);
        }

        c
    }
}
