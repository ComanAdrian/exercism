use std::fmt::Debug;

#[derive(Debug)]
pub struct CircularBuffer<T> {
    head: usize,
    values: Vec<Option<T>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
where
    T: Debug,
{
    pub fn new(capacity: usize) -> Self {
        let values: Vec<Option<T>> = (0..capacity).map(|_| None).collect();
        CircularBuffer { head: 0, values }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.values.get(self.head).unwrap().is_some() {
            return Err(Error::FullBuffer);
        } else {
            *self.values.get_mut(self.head).unwrap() = Some(element);
        }

        self.update_head();
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        for i in 0..self.capacity() {
            let mut index_to_check = self.head + i;
            index_to_check = if index_to_check < self.capacity() {
                index_to_check
            } else {
                index_to_check - self.capacity()
            };

            let elem = self.values.get_mut(index_to_check).unwrap();
            if elem.is_some() {
                let removed_element = elem.take().unwrap();
                return Ok(removed_element);
            }
        }
        Err(Error::EmptyBuffer)
    }

    pub fn clear(&mut self) {
        self.values.iter_mut().for_each(|elem| {
            elem.take();
        });
    }

    pub fn overwrite(&mut self, element: T) {
        *self.values.get_mut(self.head).unwrap() = Some(element);
        self.update_head();
    }

    fn update_head(&mut self) {
        self.head = if (self.head + 1) == self.capacity() {
            0
        } else {
            self.head + 1
        };
    }

    fn capacity(&self) -> usize {
        self.values.len()
    }
}
