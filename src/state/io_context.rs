use std::io::{BufRead, BufReader, BufWriter, Read, Write};

use crate::errors::runtime_error::RuntimeError;

pub struct IoContext<R: Read, W: Write> {
    pub reader: BufReader<R>,
    pub writer: BufWriter<W>,
}

impl<R: Read, W: Write> IoContext<R, W> {
    pub fn write_line(&mut self, s: &str) -> Result<(), RuntimeError> {
        self.writer
            .write_all(s.as_bytes())
            .map_err(|e| io_error("write", e))?;

        self.writer
            .write_all(b"\n")
            .map_err(|e| io_error("write", e))?;

        self.writer.flush().map_err(|e| io_error("flush", e))
    }

    pub fn read_line(&mut self) -> Result<String, RuntimeError> {
        let mut line = String::new();

        self.reader
            .read_line(&mut line)
            .map_err(|e| io_error("read_line", e))?;

        // Strip trailing newline and optional carriage return
        if line.ends_with('\n') {
            line.pop(); // remove '\n'
            if line.ends_with('\r') {
                line.pop(); // remove '\r' if present (Windows-style newline)
            }
        }

        Ok(line)
    }
}

fn io_error(op: &str, e: std::io::Error) -> RuntimeError {
    RuntimeError {
        message: format!("Failed to {} in IoContext: {}", op, e),
    }
}
