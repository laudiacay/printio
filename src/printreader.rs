use std::io;
use std::io::{Read, Write};

/// A reader that wraps another reader and writes the bytes read to a side effect writer (for debugging)
pub struct PrintReader<R: Read, W: Write> {
    /// The underlying reader that we are wrapping
    underlying_reader: R,
    /// The writer that we are writing to as debug output for the underlying reader
    side_effect_writer: W,
}

impl<R: Read, W: Write> PrintReader<R, W> {
    /// Create a new PrintReader that writes to the specified writer
    pub(crate) fn new(underlying_reader: R, side_effect_writer: W) -> Self {
        Self {
            underlying_reader,
            side_effect_writer,
        }
    }
}

/// Read trait implementation... easy peasy
impl<R: Read> PrintReader<R, io::Stdout> {
    /// Create a new PrintReader that writes to stdout
    pub(crate) fn new_to_stdout(underlying_reader: R) -> Self {
        Self {
            underlying_reader,
            side_effect_writer: io::stdout(),
        }
    }
}
impl<R: Read, W: Write> Read for PrintReader<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.underlying_reader.read(buf)?;
        self.side_effect_writer.write_all(&buf[..bytes_read])?;
        Ok(bytes_read)
    }
}
