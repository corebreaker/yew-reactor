use instant::{Duration, Instant};
use std::{fmt::{Display, Formatter, Result as FmtResult}, cell::RefCell};

#[derive(Debug, Clone)]
pub struct DurationInfo {
    start: RefCell<Instant>,
    duration: Duration,
}

impl DurationInfo {
    pub fn new() -> Self {
        Self {
            start: RefCell::new(Instant::now()),
            duration: Duration::default(),
        }
    }

    pub fn begin(&self) {
        let mut start = self.start.borrow_mut();

        *start = Instant::now();
    }

    pub fn end(&mut self) {
        self.duration = Instant::now().duration_since(*self.start.borrow());
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }
}

impl Display for DurationInfo {
    fn fmt(&self, f: &'_ mut Formatter) -> FmtResult {
        let duration = self.duration.as_nanos();
        let ms = duration / 1_000_000;
        let ns = format!(".{:06}", duration % 1_000_000);
        let mut ns_str = ns.trim_end_matches('0');
        if ns_str == "." {
            ns_str = "";
        }

        write!(f, "{ms}{} ms", ns_str.trim_end_matches('0'))
    }
}
