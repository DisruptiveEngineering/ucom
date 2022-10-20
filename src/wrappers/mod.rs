mod re;
mod timestamp;

use std::io::Write;

pub use re::{RegexWrapper, RegexWrapperModes};
pub use timestamp::TimestampWrapper;

pub trait WrapperBuilder {
    fn wrap(&self, drain: Box<dyn Write>) -> Box<dyn Write>;
}
