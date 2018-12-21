#![feature(test)]
extern crate test;

extern crate clap;

mod files;
pub use files::*;

mod stat_metrics;
pub use stat_metrics::*;