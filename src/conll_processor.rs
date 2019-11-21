extern crate conllx;

use conllx::{ReadSentence, Reader, Token};
use flate2::read::GzDecoder;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

/// Read single file
pub fn read_conll_file(datafile: &str) -> Vec<Vec<String>> {
    read_conll_sentences(datafile)
}

pub fn read_conll_sentences(filename: &str) -> Vec<Vec<String>> {
    if filename.ends_with(".conll.gz") {
        let reader = File::open(filename).expect("Couldn't open file");
        let boxed_reader = BufReader::new(GzDecoder::new(reader).expect("Couldn't unzip .gz file"));
        Reader::new(boxed_reader)
            .sentences()
            .map(|s| sent_to_forms(s.unwrap()))
            .collect()
    } else if filename.ends_with(".conll") {
        let reader = File::open(filename).expect("Couldn't open file");
        Reader::new(BufReader::new(reader))
            .sentences()
            .map(|s| sent_to_forms(s.unwrap()))
            .collect()
    } else {
        Vec::new()
    }
}

pub fn sent_to_forms(sent: Vec<Token>) -> Vec<String> {
    let mut forms = Vec::with_capacity(sent.len());
    for token in sent {
        forms.push(token.form().to_string())
    }
    forms
}

pub fn write_conll2txt(
    conll_sentences: &Vec<Vec<String>>,
    output_filename: &str,
) -> io::Result<()> {
    if fs::metadata(&output_filename).is_ok() {
        println!("Appending to file ({})", output_filename);
        let mut file = OpenOptions::new()
            .append(true)
            .open(output_filename)
            .unwrap();
        for sent in conll_sentences {
            for i in 0..sent.len() {
                write!(file, "{}", sent[i]);

                if i == sent.len()-1 {
                    write!(file, "\n");
                } else {
                    write!(file, " ");
                }
            }
        }
    } else {
        println!("Creating new file \"{}\"", output_filename);
        let mut file = File::create(&output_filename)?;
        for sent in conll_sentences {
            for i in 0..sent.len() {
                write!(file, "{}", sent[i]);

                if i == sent.len()-1 {
                    write!(file, "\n");
                } else {
                    write!(file, " ");
                }
            }
        }
    }

    Ok(())
}


/// Get all files from a directory the name of which is provided as string.
/// Return list of all files in directory incl. subdirectories.
pub fn get_all_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    let dir = Path::new(path);
    get_conll_files(dir, &mut files);
    files
}

/// Get all files from a directory, also recursively if necessary.
fn get_conll_files(dir: &Path, files: &mut Vec<String>) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                get_conll_files(&path, files);
            } else {
                let filename = path.to_str().unwrap().clone().to_string();
                if filename.ends_with("conll") || filename.ends_with("conll.gz") {
                    files.push(filename);
                }
            }
        }
    } else if dir.is_file() {
        let filename = dir.to_str().unwrap().clone().to_string();
        if filename.ends_with("conll") || filename.ends_with("conll.gz") {
            files.push(filename);
        }
    }
}