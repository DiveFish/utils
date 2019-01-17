extern crate clap;
extern crate utils;

use clap::{Arg, App};
use utils::{file2f64_list, f64_list2file};
use std::fs::File;

fn main() {
    let matches = App::new("utils")
        .version("1.0")
        .author("Patricia Fischer")
        .about("")
        .arg(Arg::with_name("INPUT_FILE")
            .help("Sets the input file or directory")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT_FILE")
            .help("Sets the output file or directory")
            .required(true)
            .index(2))
            .get_matches();
    //let (x_list, y_list) = file_to_f64_lists(matches.value_of("INPUT_FILE").expect("File not found"));
    //let corr = spearman(&x_list, &y_list);
    //println!("Spearman correlation: {}", corr);

    let list = file2f64_list(matches.value_of("INPUT_FILE").expect("File not found"));
    //f64_list2file(&list, matches.value_of("OUTPUT_FILE").expect("File not found"));
}