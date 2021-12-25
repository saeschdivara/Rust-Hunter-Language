mod scanning;
mod interpreter;
mod reporting;

use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::info;

use crate::interpreter::run;

fn setup_logger() -> Result<(), fern::InitError> {
    let mut colors = ColoredLevelConfig::new()
        // use builder methods
        .info(Color::Green);
    // or access raw fields
    colors.warn = Color::Magenta;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}:{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.file().unwrap(),
                record.line().unwrap(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    let result = setup_logger();

    match result {
        Err(error) => panic!("Logger setup failed: {}", error),
        _ => {}
    }

    let args: Vec<String> = env::args().collect();
    info!("Compiler arguments: {:?}", args);

    let file_name: String = args[1].clone();
    let file_content = fs::read_to_string(&file_name).expect("Something went wrong during reading");
    run(file_name, file_content);
}
