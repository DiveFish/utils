extern crate conllx;

use conllx::WriteSentence;

use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::process;

pub fn create_splits(input_dir: &Path, output_dir: &Path, splits: &[usize]) -> io::Result<()> {
    eprintln!("{:?} {:?}", input_dir, output_dir);
    let mut all_sents = Vec::new();

    if input_dir.is_dir() {
        for entry in fs::read_dir(input_dir).unwrap() {
            let path = entry.unwrap().path();
            if path
                .to_str()
                .unwrap()
                .clone()
                .to_string()
                .ends_with("conll")
            {
                if path.is_file() {
                    let reader = conllx::Reader::new(BufReader::new(
                        File::open(path).expect("Couldn't open file"),
                    ));
                    let mut sents: Vec<_> = reader
                        .into_iter()
                        .map(|r| r.expect("Could not read sentence"))
                        .collect();

                    all_sents.append(&mut sents);
                }
            }
        }
    }

    let total = all_sents.len();
    let train_size = total / 10 * splits[0];
    let validation_size = total / 10 * splits[1];
    let test_size = total - (train_size + validation_size);

    let train_file =
        "/Users/patricia/RustProjects/dpar/dpar-utils/testdata/tueba-dz/parts/7-1-2/train-7.conll";
    let validate_file = "/Users/patricia/RustProjects/dpar/dpar-utils/testdata/tueba-dz/parts/7-1-2/validation-1.conll";
    let test_file =
        "/Users/patricia/RustProjects/dpar/dpar-utils/testdata/tueba-dz/parts/7-1-2/test-2.conll";

    let mut idx = 0;
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(File::create(train_file))));
    while idx < train_size {
        writer.write_sentence(&all_sents[idx]);
        idx += 1;
    }
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(File::create(validate_file))));
    while idx < train_size + validation_size {
        writer.write_sentence(&all_sents[idx]);
        idx += 1;
    }
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(File::create(test_file))));
    while idx < train_size + validation_size + test_size {
        writer.write_sentence(&all_sents[idx]);
        idx += 1;
    }
    Ok(())
}

pub fn or_exit<T, E: fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e: E| -> T {
        println!("Error: {}", e);
        process::exit(1)
    })
}
