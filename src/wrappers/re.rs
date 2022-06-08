use std::io::Write;

pub struct RegexWrapper {
    pub out: Box<dyn Write>,
    pub re: regex::Regex,
    pub buffer: Vec<u8>,
}

impl Write for RegexWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        while let Some(s) = self.get_line() {
            // Check match on string except last byte which is '\n'
            if self.re.is_match(&s[..s.len() - 1]) {
                self.out.write_all(s.as_bytes())?;
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}

impl RegexWrapper {
    pub fn new(re: &str, out: Box<dyn Write>) -> Self {
        Self {
            out,
            re: regex::Regex::new(re).unwrap(),
            buffer: Vec::new(),
        }
    }

    pub fn get_line(&mut self) -> Option<String> {
        let n = self.buffer.iter().position(|x| x == &b'\n')?;

        let front = self.buffer.drain(0..=n);
        Some(String::from_utf8_lossy(front.as_ref()).to_string())
    }
}
