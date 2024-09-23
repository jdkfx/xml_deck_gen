mod pdf_generator;
mod xml_parser;

use pdf_generator::generate;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use xml_parser::{parse, tokenize};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Not enough arguments!");
        std::process::exit(1);
    }

    if !args[1].ends_with(".xml") {
        eprintln!("Error: Input file name is required!");
        std::process::exit(1);
    }

    let mut input_file = File::open(&args[1]).expect("Error: Input File not found!");

    let mut xml_contents = String::new();
    input_file
        .read_to_string(&mut xml_contents)
        .expect("Error: Something went wrong reading the file!");

    let re = Regex::new(r"[\t\n]").unwrap();
    xml_contents = re.replace_all(&xml_contents, "").to_string();

    let tokens = tokenize(&xml_contents);

    match parse(&tokens) {
        Ok(node) => {
            generate(node);
        }
        Err(err) => {
            eprintln!("Parsing error: {}", err);
        }
    }
}
