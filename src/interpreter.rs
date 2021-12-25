use log::info;
use crate::reporting::CodeReporter;

use crate::scanning::Scanner;

pub fn run(file_name: String, source: String) {
    let reporter = CodeReporter::new();
    let mut scanner = Scanner::new(reporter);
    let tokens = scanner.scan_string(file_name, source);

    for token in tokens {
        info!("Token: {:?}", token);
    }
}