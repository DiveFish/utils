use std::fs::File;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};

/// Read 1-column contents of a file into a set
pub fn file_to_set(filename: &str) -> HashSet<String> {
    let file = File::open(filename).expect("Could not open file");
    let mut contents = HashSet::new();
    for line in BufReader::new(file).lines() {
        contents.insert(line.expect("Could not read line"));
    }
    contents
}

/// Read 2-column contents of a file into 2 sets
pub fn file_to_sets(filename: &str) -> (HashSet<String>, HashSet<String>) {
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

/// Read 1-column contents of a file into a list
pub fn file_to_list(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Could not open file");
    let mut contents = Vec::new();
    for line in BufReader::new(file).lines() {
        contents.push(line.expect("Could not read line"));
    }
    contents
}

/// Read 2-column contents of a file into 2 lists
pub fn file_to_lists(filename: &str) -> (Vec<String>, Vec<String>) {
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


/// Read 1-column contents of a file into a list
pub fn file_to_f64_list(filename: &str) -> Vec<f64> {
    let file = File::open(filename).expect("Could not open file");
    let mut contents = Vec::new();
    for line in BufReader::new(file).lines() {
        contents.push(line.expect("Could not read line").parse::<f64>().unwrap_or(-1.0));
    }
    contents
}

/// Read 2-column contents of a file into 2 lists
pub fn file_to_f64_lists(filename: &str) -> (Vec<f64>, Vec<f64>) {
    let file = File::open(filename).expect("Could not open file");
    let mut col1_set = Vec::new();
    let mut col2_set = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Cannot read line");
        let line_split = line.split(" ").collect::<Vec<_>>();
        col1_set.push(line_split[0].to_string().parse::<f64>().unwrap_or(-1.0));
        col2_set.push(line_split[1].to_string().parse::<f64>().unwrap_or(-1.0));
    }

    (col1_set, col2_set)
}