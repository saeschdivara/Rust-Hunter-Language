use chrono::Local;
use log::info;
use crate::reporting::CodeReporter;

use crate::scanning::Scanner;

pub fn run(file_name: String, source: String) {
    let reporter = CodeReporter::new();
    let mut scanner = Scanner::new(reporter);

    let start = Local::now().time();
    let tokens = scanner.scan_string(file_name, source);
    let end = Local::now().time();

    info!("Parsing took {} ms", (end - start).num_milliseconds());

    for token in tokens {
        info!("Token: {:?}", token);
    }
}