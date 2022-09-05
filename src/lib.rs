use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;
const INIT_PADDING: usize = 6;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut line_number = 1;
                for line_result in file.lines() {
                    match line_result {
                        Ok(line) => {
                            if config.number_lines {
                                print_number(&line, line_number);
                                line_number += 1;
                            } else if config.number_nonblank_lines {
                                print_number_nonblank(&line, line_number);
                                line_number += number_nonblank_incr(&line);
                            } else {
                                println!("{}", line)
                            }
                        }
                        Err(err) => eprintln!("{}", err),
                    }
                }
            }
        }
    }
    Ok(())
}

fn print_number_nonblank(line: &String, line_number: usize) {
    let padding = count_digits(line_number);
    if line != "" {
        println!(
            "{}{}\t{}",
            " ".repeat(INIT_PADDING - padding),
            line_number,
            line
        );
    } else {
        println!();
    }
}

fn number_nonblank_incr(line: &String) -> usize {
    if line != "" {
        1
    } else {
        0
    }
}

fn print_number(line: &String, line_number: usize) {
    let padding = count_digits(line_number);
    println!(
        "{}{}\t{}",
        " ".repeat(INIT_PADDING - padding),
        line_number,
        line
    );
}

fn count_digits(n: usize) -> usize {
    let mut digits = 1;
    let mut tens: usize = 10;

    while n >= tens {
        digits += 1;
        tens *= 10;
    }

    digits
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Kevin Ringier <kevinringier@gmail.com>")
        .about("Concatenate FILE(s) to standard output.")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("With no FILE, or when FILE is -, read standard input.")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number all output lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n \nequivalent to -vE")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => (Ok(Box::new(BufReader::new(io::stdin())))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
