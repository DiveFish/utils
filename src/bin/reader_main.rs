extern crate clap;
extern crate rust2vec;
extern crate stdinout;
extern crate utils;

use std::fs::File;
use std::io::BufWriter;
use stdinout::OrExit;
use clap::{App, Arg};

use utils::*;

fn main() {
    let matches = App::new("utils")
        .version("1.0")
        .author("Patricia Fischer")
        .about("")
        .arg(
            Arg::with_name("INPUT_DIR")
                .help("Sets the input file or directory")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT_DIR")
                .help("Sets the output file or directory")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name("SPLITS")
                .help("Sets the splits for the train/validate/test sets")
                .required(false)
                .index(3),
        )
        .get_matches();

    let input_dir = matches
        .value_of("INPUT_DIR")
        .expect("Could not read input directory");
    let output_dir = matches
        .value_of("OUTPUT_DIR")
        .expect("Could not read output directory");

    let text = read_conll_file_to_string(input_dir);
    write_conll2txt(&text, output_dir);
}