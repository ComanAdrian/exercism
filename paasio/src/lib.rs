use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    operations: usize,
    bytes_through: usize
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            operations: 0,
            bytes_through: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.operations
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let result = self.wrapped.read(buf)?;
        self.operations += 1;
        self.bytes_through += result;
        Ok(result)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    operations: usize,
    bytes_through: usize
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            operations: 0,
            bytes_through: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.operations
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let result = self.wrapped.write(buf)?;
        self.operations += 1;
        self.bytes_through += result;

        Ok(result)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
