use std::io;
use std::io::{Read, Write};

/// A reader that wraps another reader and writes the bytes read to a side effect writer (for debugging)
pub struct PrintReader<R: Read, W: Write> {
    /// The underlying reader that we are wrapping
    underlying_reader: R,
    /// The writer that we are writing to as debug output for the underlying reader
    side_effect_writer: W,
    /// label for this printreader
    label: String,
}

impl<R: Read, W: Write> PrintReader<R, W> {
    /// Create a new PrintReader that writes to the specified writer
    pub fn new(underlying_reader: R, side_effect_writer: W) -> Self {
        Self {
            underlying_reader,
            side_effect_writer,
            label: "".parse().unwrap(),
        }
    }

    pub fn new_with_label(
        underlying_reader: R,
        side_effect_writer: W,
        label: String,
    ) -> Self {
        Self {
            underlying_reader,
            side_effect_writer,
            label,
        }
    }
}

/// Read trait implementation... easy peasy
impl<R: Read> PrintReader<R, io::Stdout> {
    /// Create a new PrintReader that writes to stdout
    pub fn new_to_stdout(underlying_reader: R) -> Self {
        Self {
            underlying_reader,
            side_effect_writer: io::stdout(),
            label: "".parse().unwrap(),
        }
    }
    pub fn new_to_stdout_with_label(underlying_reader: R, label: String) -> Self {
        Self {
            underlying_reader,
            side_effect_writer: io::stdout(),
            label,
        }
    }
}
impl<R: Read, W: Write> Read for PrintReader<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.underlying_reader.read(buf)?;
        self.side_effect_writer.write_all(self.label.as_ref())?;
        self.side_effect_writer.write_all(&buf[..bytes_read])?;
        Ok(bytes_read)
    }
}
