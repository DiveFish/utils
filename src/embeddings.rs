use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::mem;
use std::slice::from_raw_parts_mut;
use stdinout::OrExit;

use failure::{err_msg, Error};
use ndarray::{Array2, Axis};
use rust2vec::embeddings::Embeddings;
use rust2vec::prelude::ReadWord2Vec;
use rust2vec::storage::NdArray;
use rust2vec::vocab::SimpleVocab;

pub fn read_w2v(filename: &str) -> Result<rust2vec::embeddings::Embeddings<SimpleVocab, NdArray>, Error> {
    let mut reader = BufReader::new(File::open(filename).or_exit("Could not open file", 1));
    let mut check = Vec::new();
    reader.read_to_end(&mut check).or_exit("Could not read to end", 1);

    // Read embeddings.
    reader.seek(SeekFrom::Start(0)).or_exit("Could not start at 0", 1);
    Embeddings::read_word2vec_binary(&mut reader, false)
}

pub fn load_w2v_embeddings(
    input_filename: &str,
) -> Result<rust2vec::embeddings::Embeddings<SimpleVocab, NdArray>, Error> {
    let f = File::open(input_filename).expect("Cannot read file");
    let mut reader = BufReader::new(f);

    let n_words = read_number(&mut reader, b' ', 0).or_exit("Problems reading n_words", 1);
    let embed_len = read_number(&mut reader, b'\n', 0).or_exit("Problems reading embed_len", 1);

    let mut matrix = Array2::<f32>::zeros((n_words, embed_len));
    let mut words = Vec::with_capacity(n_words);

    for idx in 0..n_words {
        // Use ' ' for word-embedding split on _ and '\t' for split on tab
        let word = read_string(&mut reader, b'\t', idx)
            .or_exit(format!("Could not read word at idx {}", idx), 1);
        let word = word.trim();
        words.push(word.to_owned());
        remove_space(&mut reader, b' ');
        let mut embedding = matrix.index_axis_mut(Axis(0), idx);

        {
            let mut embedding_raw = match embedding.as_slice_mut() {
                Some(s) => unsafe { typed_to_bytes(s) },
                None => return Err(err_msg("Matrix not contiguous")),
            };
            reader
                .read_exact(&mut embedding_raw)
                .or_exit("Could not read exact embedding", 1);
        }
    }

    Ok(Embeddings::new(
        None,
        SimpleVocab::new(words),
        NdArray(matrix),
    ))
}

fn read_number(reader: &mut BufRead, delim: u8, idx: usize) -> Result<usize, Error> {
    let field_str = read_string(reader, delim, idx)?;
    Ok(field_str.parse()?)
}

fn read_string(reader: &mut BufRead, delim: u8, idx: usize) -> Result<String, Error> {
    let mut buf = Vec::new();
    reader.read_until(delim, &mut buf)?;
    buf.pop();
    Ok(String::from_utf8(buf)?)
}

fn remove_space(reader: &mut BufRead, delim: u8) -> Result<(), Error> {
    let mut buf = vec![];
    reader.read_until(delim, &mut buf);
    Ok(())
}

unsafe fn typed_to_bytes<T>(slice: &mut [T]) -> &mut [u8] {
    from_raw_parts_mut(
        slice.as_mut_ptr() as *mut u8,
        slice.len() * mem::size_of::<T>(),
    )
}
