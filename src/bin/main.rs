extern crate clap;
extern crate rust2vec;
extern crate stdinout;
extern crate utils;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use stdinout::OrExit;

use clap::{App, Arg};
use rust2vec::embeddings::Embeddings;
use rust2vec::io::WriteEmbeddings;
use rust2vec::prelude::WriteWord2Vec;

use utils::{load_w2v_embeddings, read_w2v};

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
    //let (x_list, y_list) = file_to_f64_lists(matches.value_of("INPUT_FILE").expect("File not found"));
    //let corr = spearman(&x_list, &y_list);
    //println!("Spearman correlation: {}", corr);

    //let list = create_splits(matches.value_of("INPUT_FILE").expect("File not found"));
    //f64_list2file(&list, matches.value_of("OUTPUT_FILE").expect("File not found"));

    let input_dir = matches
        .value_of("INPUT_DIR")
        .expect("Could not read directory");

    let output_dir = matches
        .value_of("OUTPUT_DIR")
        .expect("Could not read directory");
    /*
    let splits: Vec<usize> = matches
        .value_of("SPLITS")
        .expect("Could not read splits")
        .split("-")
        .map(|tag| tag.trim().to_owned().parse::<usize>().unwrap())
        .collect();
    create_splits(Path::new(input_dir), Path::new(output_dir), &splits);
    */
    let embeddings = load_w2v_embeddings(input_dir).or_exit("Cannot read from embeddings file", 1);
    //let embeddings = read_w2v(input_dir).or_exit("Cannot read from embeddings file", 1);

    let f = File::create(output_dir).or_exit("Cannot write to embeddings file", 1);

    let mut writer = BufWriter::new(f);
    if output_dir.ends_with("bin") {
        embeddings
            .write_word2vec_binary(&mut writer)
            .or_exit("Cannot write embedding to file", 1);
    } else if output_dir.ends_with("fifu") {
        embeddings
            .write_embeddings(&mut writer)
            .or_exit("Cannot write embedding to file", 1);
    } else {
        eprintln!("Provide either bin or fifu output");
    }
}
