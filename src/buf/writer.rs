use crate::BufMut;
use core::cmp;
use embedded_io::ErrorType;

/// A `BufMut` adapter which implements `embedded_io::Write` for the inner value.
///
/// This struct is generally created by calling `writer()` on `BufMut`.
#[derive(Debug)]
pub struct Writer<B> {
    buf: B,
}

/// Creates a new `Writer` for the given buffer.
pub fn new<B>(buf: B) -> Writer<B> {
    Writer { buf }
}

impl<B: BufMut> Writer<B> {
    /// Gets a reference to the underlying `BufMut`.
    pub fn get_ref(&self) -> &B {
        &self.buf
    }

    /// Gets a mutable reference to the underlying `BufMut`.
    pub fn get_mut(&mut self) -> &mut B {
        &mut self.buf
    }

    /// Consumes this `Writer`, returning the underlying value.
    pub fn into_inner(self) -> B {
        self.buf
    }
}

// Implement `ErrorType` for the `Writer`.
#[derive(Debug)]
pub struct WriterError;

impl embedded_io::Error for WriterError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl<B: BufMut + Sized> ErrorType for Writer<B> {
    type Error = WriterError;
}

// Implement `embedded_io::Write` for the `Writer`.
impl<B: BufMut + Sized> embedded_io::Write for Writer<B> {
    fn write(&mut self, src: &[u8]) -> Result<usize, Self::Error> {
        let n = cmp::min(self.buf.remaining_mut(), src.len());
        self.buf.put_slice(&src[..n]);
        Ok(n)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
