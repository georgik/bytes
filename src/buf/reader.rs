use crate::Buf;
use core::cmp;
use embedded_io::ErrorType;

// Define a custom error type for the Reader
#[derive(Debug)]
pub struct ReaderError;

// Implement ErrorType for ReaderError
impl embedded_io::Error for ReaderError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

/// A `Buf` adapter which implements `embedded_io::Read` for the inner value.
#[derive(Debug)]
pub struct Reader<B> {
    buf: B,
}

pub fn new<B>(buf: B) -> Reader<B> {
    Reader { buf }
}

impl<B: Buf> Reader<B> {
    pub fn get_ref(&self) -> &B {
        &self.buf
    }

    pub fn get_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    pub fn into_inner(self) -> B {
        self.buf
    }
}

// Implement ErrorType for Reader<B>
impl<B: Buf + Sized> ErrorType for Reader<B> {
    type Error = ReaderError;
}

impl<B: Buf + Sized> embedded_io::Read for Reader<B> {
    fn read(&mut self, dst: &mut [u8]) -> Result<usize, Self::Error> {
        let len = cmp::min(self.buf.remaining(), dst.len());
        Buf::copy_to_slice(&mut self.buf, &mut dst[0..len]);
        Ok(len)
    }
}
