#![feature(test)]
extern crate test;

#[macro_use]
extern crate failure;

extern crate clap;

extern crate conllx;

extern crate ndarray;

extern crate rust2vec;

extern crate stdinout;

extern crate tensorflow;

extern crate tf_embed;

mod conllx_splitter;
pub use conllx_splitter::*;

mod embeddings;
pub use embeddings::*;

mod files;
pub use files::*;

mod stat_metrics;
pub use stat_metrics::*;
