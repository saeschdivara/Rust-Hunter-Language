use chrono::Local;
use log::info;
use crate::parser::Parser;
use crate::reporter::CodeReporter;

use crate::scanner::Scanner;

pub fn run(file_name: String, source: String) {
    let reporter = CodeReporter::new();
    let mut scanner = Scanner::new(reporter);

    let start_scanning = Local::now().time();
    let tokens = scanner.scan_string(file_name, source);
    let end_scanning = Local::now().time();

    info!("Scanning took {} ms", (end_scanning - start_scanning).num_milliseconds());

    for token in &tokens {
        info!("Token: {:?}", token);
    }

    let mut parser = Parser::new(tokens);

    let start_parser = Local::now().time();
    let ast = parser.parse_ast();
    let end_parser = Local::now().time();

    info!("Parsing took {} ms", (end_parser - start_parser).num_milliseconds());

    ast.dump();
}