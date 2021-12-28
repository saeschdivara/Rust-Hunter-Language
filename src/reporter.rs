use log::warn;

#[derive(Clone, Copy)]
pub struct CodeReporter {
    has_error: bool
}

impl CodeReporter {

    pub fn new() -> Self {
        CodeReporter { has_error: false }
    }

    pub fn report_error(&mut self, file: &String, line: usize, message: &String) {
        self.has_error = true;
        self.report(file, line, message);
    }

    fn report(&self, file: &String, line: usize, message: &String) {
        warn!("[{}:{}]: {}", file, line, message);
    }
}