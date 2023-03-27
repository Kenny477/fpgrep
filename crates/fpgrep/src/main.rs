use clap::Parser;
use log::{debug, error, info, trace, warn};
use regex::Regex;
use std::{io::Read, process::exit, str::pattern};

/* Define exit codes
    line selected = 0
    no line selected = 1
    error = 2
*/
const SELECTED: i32 = 0;
const NO_SELECTED: i32 = 1;
const ERROR: i32 = 2;

/* CLI syntax
fpgrep [OPTIONS...] [PATTERN] [FILES...]
 */
#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    files: Vec<std::path::PathBuf>,
}

fn search(pattern: &Regex, buffer: &str) {
    let mut lines = buffer.lines();
    let mut line_num = 0;
    while let Some(line) = lines.next() {
        line_num += 1;
        debug!("{}: {}", line_num, line);
        if pattern.is_match(line) {
            println!("{}:{}", line_num, line);
        }
    }
}

fn handle_stdin(buffer: &mut String, pattern: &Regex) {
    match atty::is(atty::Stream::Stdin) {
        true => {
            debug!("No input given...reading from stdin");
            let stdin = std::io::stdin();
            let mut stdin = stdin.lock();
            stdin.read_to_string(buffer).unwrap();
        }
        false => {
            debug!("Reading from stdin");
            std::io::stdin().read_to_string(buffer).unwrap();
        }
    }
    debug!("{}", buffer);
    search(&pattern, &buffer);
}

fn handle_files(files: &Vec<std::path::PathBuf>, pattern: &Regex) {
    for file in files {
        debug!("Reading from file: {:?}", file);
        let mut file = match std::fs::File::open(file) {
            Ok(file) => file,
            Err(e) => {
                error!("Error opening file: {}", e);
                exit(ERROR);
            }
        };
        file.read_to_string(&mut buffer).unwrap();
        debug!("{}", buffer);
        search(&pattern, &buffer);
    }
}

fn main() {
    let args = Cli::parse();
    debug!("{:?}", args);

    /* Check if pattern is valid regex */
    let pattern = match Regex::new(&args.pattern) {
        Ok(pattern) => pattern,
        Err(e) => {
            error!("Invalid regex: {}", e);
            exit(ERROR);
        }
    };

    /* Read from stdin or files */
    let mut buffer: String = String::new();
    if args.files.is_empty() {
        debug!("No files given...reading from stdin");
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        stdin.read_to_string(&mut buffer).unwrap();
        debug!("{}", buffer);
        search(&pattern, &buffer)
    } else {
        for file in args.files {
            debug!("Reading from file: {:?}", file);
            let mut file = match std::fs::File::open(file) {
                Ok(file) => file,
                Err(e) => {
                    error!("Error opening file: {}", e);
                    exit(ERROR);
                }
            };
            file.read_to_string(&mut buffer).unwrap();
            debug!("{}", buffer);
            search(&pattern, &buffer);
        }
    }
}
