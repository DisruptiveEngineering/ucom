use crate::wrappers::WrapperBuilder;
use std::io::Write;

#[derive(Copy, Clone)]
pub enum RegexWrapperModes {
    Match,
    Filter,
}

pub struct RegexWrapper {
    pub re: regex::Regex,
    pub mode: RegexWrapperModes,
}

impl RegexWrapper {
    pub fn new(re: &str, mode: RegexWrapperModes) -> Self {
        Self {
            re: regex::Regex::new(re).unwrap(),
            mode,
        }
    }
}

impl WrapperBuilder for RegexWrapper {
    fn wrap(&self, drain: Box<dyn Write>) -> Box<dyn Write> {
        Box::new(Wrapper {
            out: drain,
            re: self.re.clone(),
            buffer: Vec::new(),
            mode: self.mode,
        })
    }
}

struct Wrapper {
    pub out: Box<dyn Write>,
    pub re: regex::Regex,
    pub buffer: Vec<u8>,
    pub mode: RegexWrapperModes,
}

impl Write for Wrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        while let Some(s) = self.get_line() {
            // Check match on string except last byte which is '\n'
            match (self.mode, self.re.is_match(&s[..s.len() - 1])) {
                (RegexWrapperModes::Filter, false) => {
                    self.out.write_all(s.as_bytes())?;
                }
                (RegexWrapperModes::Match, true) => {
                    self.out.write_all(s.as_bytes())?;
                }
                (_, _) => {}
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}

impl Wrapper {
    pub fn get_line(&mut self) -> Option<String> {
        let n = self.buffer.iter().position(|x| x == &b'\n')?;

        let front = self.buffer.drain(0..=n);
        Some(String::from_utf8_lossy(front.as_ref()).to_string())
    }
}
