use std::io;
use std::io::Write;

/// A writer that wraps another writer and writes the bytes written to a side effect writer (for debugging)
pub struct PrintWriter<W1: Write, W2: Write> {
    /// The underlying writer that we are wrapping
    underlying_writer: W1,
    /// The writer that we are writing to as debug output for the underlying writer
    side_effect_writer: W2,
    /// label for this printwriter
    label: String,
}

impl<W1: Write, W2: Write> PrintWriter<W1, W2> {
    /// Create a new PrintWriter that writes to the specified writer
    pub(crate) fn new(underlying_writer: W1, side_effect_writer: W2) -> Self {
        Self {
            underlying_writer,
            side_effect_writer,
            label: "".parse().unwrap(),
        }
    }
    pub(crate) fn new_with_label(
        underlying_writer: W1,
        side_effect_writer: W2,
        label: String,
    ) -> Self {
        Self {
            underlying_writer,
            side_effect_writer,
            label,
        }
    }
}

impl<W: Write> PrintWriter<W, io::Stdout> {
    /// Create a new PrintWriter that writes to stdout
    pub(crate) fn new_to_stdout(underlying_writer: W) -> Self {
        Self {
            underlying_writer,
            side_effect_writer: io::stdout(),
            label: "".parse().unwrap(),
        }
    }
    pub(crate) fn new_to_stdout_with_label(underlying_writer: W, label: String) -> Self {
        Self {
            underlying_writer,
            side_effect_writer: io::stdout(),
            label,
        }
    }
}

/// Write trait implementation... easy peasy
impl<W1: Write, W2: Write> Write for PrintWriter<W1, W2> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_written = self.underlying_writer.write(buf)?;
        self.side_effect_writer.write_all(self.label.as_ref())?;
        self.side_effect_writer.write_all(&buf[..bytes_written])?;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.underlying_writer.flush()?;
        self.side_effect_writer.flush()?;
        Ok(())
    }
}
