extern crate bincode;

use std::fs::File;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::io::Write;
use std::fs;
use std::fs::OpenOptions;
use std::error::Error;
use std::io;

/// Read 1-column contents of a file into a set.
pub fn file2set(filename: &str) -> HashSet<String> {
    let file = File::open(filename).expect("Could not open file");
    let mut contents = HashSet::new();
    for line in BufReader::new(file).lines() {
        contents.insert(line.expect("Could not read line"));
    }
    contents
}

/// Read 2-column contents of a file into two sets.
pub fn file2sets(filename: &str) -> (HashSet<String>, HashSet<String>) {
    let file = File::open(filename).expect("Could not open file");
    let mut col1_set = HashSet::new();
    let mut col2_set = HashSet::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Cannot read line");
        let line_split = line.split(" ").collect::<Vec<_>>();
        col1_set.insert(line_split[0].to_string());
        col2_set.insert(line_split[1].to_string());
    }

    (col1_set, col2_set)
}

/// Read 1-column contents of a file into a list.
pub fn file2list_str(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Could not open file");
    let mut contents = Vec::new();
    for line in BufReader::new(file).lines() {
        contents.push(line.expect("Could not read line"));
    }
    contents
}

/// Read 2-column contents of a file into two lists.
pub fn file2lists_str(filename: &str) -> (Vec<String>, Vec<String>) {
    let file = File::open(filename).expect("Could not open file");
    let mut col1_set = Vec::new();
    let mut col2_set = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Cannot read line");
        let line_split = line.split(" ").collect::<Vec<_>>();
        col1_set.push(line_split[0].to_string());
        col2_set.push(line_split[1].to_string());
    }

    (col1_set, col2_set)
}

/// Read 1-column contents of a file into a list.
pub fn file2f64_list(filename: &str) -> Vec<f64> {
    let file = File::open(filename).expect("Could not open file");
    let mut contents = Vec::new();
    for line in BufReader::new(file).lines() {
        contents.push(line.expect("Could not read line").parse::<f64>().expect("Could not parse value"));
    }
    contents
}

/// Read 2-column contents of a file into two lists.
pub fn file2f64_lists(filename: &str) -> (Vec<f64>, Vec<f64>) {
    let file = File::open(filename).expect("Could not open file");
    let mut col1_set = Vec::new();
    let mut col2_set = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Could not read line");
        let line_split = line.split(" ").collect::<Vec<_>>();
        col1_set.push(line_split[0].to_string().parse::<f64>().expect("Could not parse value"));
        col2_set.push(line_split[1].to_string().parse::<f64>().expect("Could not parse value"));
    }

    (col1_set, col2_set)
}

/// Write list of floats to binary file.
pub fn f64_list2file(float_list: &[f64], filename: &str) -> io::Result<()> {

    if ! filename.ends_with("bin") {
        eprintln!("Provide binary file name");
    }

    if fs::metadata(&filename).is_ok() {
        println!("Appending to file ({})", filename);
        let mut file = OpenOptions::new().append(true).open(filename).unwrap();
        for float in float_list {
            file.write_all(float.to_string().as_bytes());
        }
    } else {
        println!("Creating new file \"{}\"", filename);
        let mut file = File::create(&filename)?;
        for float in float_list {
            file.write_all(float.to_string().as_bytes());
        }
    }
    Ok(())
}

/// Write **compressed** list of floats to binary file.
pub fn f64_list2comprbin(float_list: &[f64], filename: &str) -> Result<(), Box<Error>> {
    let bytes: Vec<u8> = bincode::serialize(float_list)?;
    let mut file = File::create(filename)?;
    file.write_all(&bytes).map_err(|e| e.into())
}