use std::cell::RefCell;
use std::fmt;
use std::ops::Rem;

pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString,
    {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

pub struct Fizzy<T>
where
    T: From<u8> + Rem<Output = T> + PartialEq + fmt::Display + Copy,
{
    // phantom: std::marker::PhantomData<T>,
    matchers: RefCell<Vec<Matcher<T>>>,
}

impl<T> Fizzy<T>
where
    T: From<u8> + Rem<Output = T> + PartialEq + fmt::Display + Copy,
{
    pub fn new() -> Self {
        Fizzy {
            // phantom: PhantomData,
            matchers: RefCell::new(vec![]),
        }
    }

    pub fn add_matcher(self, matcher: Matcher<T>) -> Self {
        // using interior mutability pattern,
        // `matcher` field looks immutable to the user, but we mutate it internally
        self.matchers.borrow_mut().push(matcher);
        Fizzy {
            matchers: self.matchers,
        }
    }

    pub fn apply(self, iter: impl Iterator<Item = T>) -> impl Iterator<Item = String> {
        // was stucked on `move` keyword, the error is very opaque if removing it
        iter.map(move |n| {
            let s = self
                .matchers
                .borrow_mut()
                .iter()
                .filter(|m| (m.matcher)(n))
                .map(|m| m.subs.clone())
                .collect::<String>();

            if s.is_empty() {
                n.to_string()
            } else {
                s.to_owned()
            }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: From<u8> + Rem<Output = T> + PartialEq + fmt::Display + Copy,
{
    let mut fizzy = Fizzy::new();
    fizzy = fizzy.add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"));
    fizzy = fizzy.add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"));

    fizzy
}
