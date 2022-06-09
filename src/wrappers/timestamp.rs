use crate::WrapperBuilder;
use std::io::Write;

pub struct TimestampWrapper;

impl WrapperBuilder for TimestampWrapper {
    fn wrap(&self, drain: Box<dyn Write>) -> Box<dyn Write> {
        Box::new(Wrapper {
            out: drain,
            buffer: Vec::new(),
        })
    }
}

struct Wrapper {
    pub out: Box<dyn Write>,
    pub buffer: Vec<u8>,
}

impl Write for Wrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.clear();
        for byte in buf {
            self.buffer.push(*byte);

            // Add timestamp
            if byte == &b'\n' {
                let now = chrono::offset::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f%:z");
                self.buffer
                    .extend_from_slice(format!("[{}] ", now).as_bytes());
            }
        }
        self.out.write_all(self.buffer.as_slice())?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}
