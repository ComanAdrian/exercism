use std::fmt::Debug;

pub struct CircularBuffer<T>
where
    T: Debug + Copy,
{
    capacity: usize,
    oldest: usize,
    head: usize,
    values: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T>
where
    T: Debug + Copy,
{
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            capacity,
            oldest: 0,
            head: 0,
            values: Vec::with_capacity(3) as Vec<T>,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        println!("write: {:?}", self.values.len());
        self.head += 1;
        self.head %= self.capacity;
        println!("write: {:?}", self.values);
        Ok(self.values.push(element))
    }

    pub fn read(&mut self) -> Result<T, Error> {
        let value = *self.values.get(self.oldest).unwrap();
        self.values.remove(self.oldest);
        self.oldest += 1;
        self.oldest %= self.capacity;
        println!("read: {:?}", self.values);
        Ok(value)
        // unimplemented!("Read the oldest element from the CircularBuffer or return EmptyBuffer error if CircularBuffer is empty.");
    }

    pub fn clear(&mut self) {
        unimplemented!("Clear the CircularBuffer.");
    }

    pub fn overwrite(&mut self, _element: T) {
        unimplemented!("Write the passed element to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.");
    }
}
