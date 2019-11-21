use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::mem;
use std::slice::from_raw_parts_mut;
use stdinout::OrExit;

use byteorder::{LittleEndian,WriteBytesExt};
use failure::{err_msg, Error};
use ndarray::{Array2, Axis};
use rust2vec;
use rust2vec::embeddings;
use rust2vec::embeddings::Embeddings;
use rust2vec::io::{WriteEmbeddings, ReadEmbeddings};
use rust2vec::prelude::{ReadWord2Vec, Vocab, WriteWord2Vec};
use rust2vec::storage::NdArray;
use rust2vec::storage::StorageWrap;
use rust2vec::vocab::SimpleVocab;
use rust2vec::vocab::VocabWrap;

/// Converts binary embeddings into finalfusion embeddings.
pub fn bin_to_fifu(output_dir: &str, embeddings: Embeddings<SimpleVocab, NdArray>) {
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

/// Embedding reader for fifu embeddings.
pub fn load_fifu_embeddings(
    input_filename: &str,
) -> Result<rust2vec::embeddings::Embeddings<SimpleVocab, NdArray>, Error> {
    let mut reader = BufReader::new(File::open(input_filename).unwrap());
    Ok(Embeddings::read_embeddings(&mut reader).unwrap())
}

/// Conventional word2vec embedding reader.
pub fn read_w2v(filename: &str) -> Result<Embeddings<SimpleVocab, NdArray>, Error> {
    let mut reader = BufReader::new(File::open(filename).or_exit("Could not open file", 1));
    let mut check = Vec::new();
    reader
        .read_to_end(&mut check)
        .or_exit("Could not read to end", 1);

    // Read embeddings.
    reader
        .seek(SeekFrom::Start(0))
        .or_exit("Could not start at 0", 1);
    Embeddings::read_word2vec_binary(&mut reader, false)
}

/// Add embeddings for null and unknown token to w2v embeddings.
pub fn adjust_w2v_embeddings(
    input_filename: &str,
) -> Result<rust2vec::embeddings::Embeddings<SimpleVocab, NdArray>, Error> {
    let f = File::open(input_filename).expect("Cannot read file");
    let mut reader = BufReader::new(f);

    let n_words = read_number(&mut reader, b' ', 0).or_exit("Problems reading n_words", 1);
    let embed_len = read_number(&mut reader, b'\n', 0).or_exit("Problems reading embed_len", 1);

    let mut matrix = Array2::<f32>::zeros((n_words + 2, embed_len));
    let mut words = Vec::with_capacity(n_words + 2);

    let mut unknown_embed = Vec::new();
    for idx in 0..n_words {
        let word = read_string(&mut reader, b' ', idx)
            .or_exit(format!("Could not read word at idx {}", idx), 1);
        let word = word.trim();
        words.push(word.to_owned());
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
        if idx == 0 {
            for i in 0..embedding.len() {
                unknown_embed.push(embedding[i]);
            }
        } else {
            for i in 0..embedding.len() {
                unknown_embed[i] += embedding[i];
            }
        }
    }
    {
        let mut unknown_embed_vec = matrix.index_axis_mut(Axis(0), n_words + 1);
        for idx in 0..unknown_embed_vec.len() {
            unknown_embed_vec[idx] += unknown_embed[idx] / n_words as f32;
        }
    }

    words.push("<NULL-TOKEN>".to_string()); // Do not modify embedding of null token, cells should remain 0
    words.push("<UNKNOWN-TOKEN>".to_string()); // Embedding is the average of all embeddings


    let embeddings = Embeddings::new(
        None,
        SimpleVocab::new(words),
        NdArray(matrix),
    );

    /*
    for (word, embeds) in embeddings.iter() {
        print!("{:?} ", word);
        for i in embeds.into_owned().iter() {
            print!("{} ", i);
        }
        println!();
    }
    */

    Ok(embeddings)
}

/// Embedding reader for word2vec embeddings where word and embedding are not split by a space.
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
        // Comment out for word-embedding split on _ and in for split on tab
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

pub fn read_w2v_vocab(filename: &str) -> Result<Vec<String>, Error> {
    let f = File::open(filename).expect("Cannot read file");
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
        words.push(word.to_string());
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
    Ok(words)
}

pub fn n_most_sim_embeds(
    word: &str,
    n: usize,
    focus_embed_file: &str,
    context_embed_file: &str,
) -> Result<(), Error> {
    let focus_embeds =
        load_w2v_embeddings(focus_embed_file).or_exit("Cannot read from embeddings file", 1);
    let context_embeds =
        load_w2v_embeddings(context_embed_file).or_exit("Cannot read from embeddings file", 1);
    let context_vocab =
        read_w2v_vocab(context_embed_file).or_exit("Cannot read vocab from embeddings file", 1);

    let word_embed = focus_embeds
        .embedding(word)
        .or_exit("Could not retrieve embedding", 1)
        .into_owned();
    let mut word_dps: Vec<(String, f32)> = Vec::with_capacity(context_vocab.len());
    for v in context_vocab {
        let cmp_embed = context_embeds
            .embedding(&v)
            .or_exit("Could not retrieve embedding", 1);
        let dp = cmp_embed.into_owned().dot(&word_embed);
        let sigmoid = 1f32 / (1f32 + (dp * (-1f32)).exp());
        word_dps.push((v, sigmoid));
    }

    //let mut word_dp_vec: Vec<(&str, &usize)> = word_dps.iter().collect();
    word_dps.sort_by(|a, b| b.1.partial_cmp(&a.1).or_exit("No dp available", 1));

    println!("Most similar contexts to {}:", word);
    for i in 0..n {
        println!("{} {}\t", word_dps[i].0, word_dps[i].1);
    }

    Ok(())
}

pub fn get_vocab_size_fifu(filename: &str) -> usize {

    let mut reader = BufReader::new(File::open(filename).unwrap());

    let embeddings: Embeddings<VocabWrap, StorageWrap> =
        Embeddings::read_embeddings(&mut reader).unwrap();

    let embeds = Embeddings::from(embeddings);

    embeds.vocab().len()
}

pub fn cmp_embeds(
    focus_words: Vec<String>,
    context_words: Vec<String>,
    focus_embed_file: &str,
    context_embed_file: &str,
) -> Result<(), Error> {
    assert_eq!(focus_words.len(), context_words.len());

    let focus_embeds = File::open(&focus_embed_file)?;
    let context_embeds = File::open(&context_embed_file)?;

    let mut focus_reader = BufReader::new(&focus_embeds);
    let focus_embeds: Embeddings<VocabWrap, StorageWrap> =
        embeddings::Embeddings::read_embeddings(&mut focus_reader).unwrap();
    println!("Loaded focus embeddings");

    let mut context_reader = BufReader::new(&context_embeds);
    let context_embeds: Embeddings<VocabWrap, StorageWrap> =
        embeddings::Embeddings::read_embeddings(&mut context_reader).unwrap();
    println!("Loaded context embeddings");

    for idx in 0..focus_words.len() {
        let focus_word = &focus_words[idx];
        let context_word = &context_words[idx];

        let focus_embedding = focus_embeds.embedding(focus_word);
        let context_embedding = context_embeds.embedding(context_word);

        if ! focus_embedding.is_some() {
            println!("No focus embedding for {}", focus_word);
        }
        if ! context_embedding.is_some() {
            println!("No context embedding for {}", focus_word);
        }

        if let (Some(focus_embedding), Some(context_embedding)) =
            (focus_embedding, context_embedding)
        {
            let dp = focus_embedding
                .into_owned()
                .dot(&context_embedding.into_owned());
            let sigmoid = 1f32 / (1f32 + (dp * (-1f32)).exp());
            println!("EBS of {} {}: {}", focus_word, context_word, sigmoid);
        }
    }

    Ok(())
}

pub fn cmp_embeds_broken(
    focus_words: Vec<String>,
    context_words: Vec<String>,
    focus_embed_file: &str,
    context_embed_file: &str,
) -> Result<(), Error> {
    assert_eq!(focus_words.len(), context_words.len());
    let focus_embeds =
        load_w2v_embeddings(focus_embed_file).or_exit("Cannot read from embeddings file", 1);
    println!("Loaded focus embeddings");
    let context_embeds =
        load_w2v_embeddings(context_embed_file).or_exit("Cannot read from embeddings file", 1);
    println!("Loaded context embeddings");

    for idx in 0..focus_words.len() {
        let focus_word = &focus_words[idx];
        let context_word = &context_words[idx];

        let focus_embed = focus_embeds.embedding(focus_word);
        let context_embed = focus_embeds.embedding(context_word);

        if !focus_embed.is_some() {
            println!("Could not retrieve focus embedding for {}", focus_word);
        }
        if !context_embed.is_some() {
            println!("Could not retrieve context embedding for {}", context_word);
        }

        if focus_embed.is_some() && context_embed.is_some() {
            let focus_embed = focus_embed.unwrap().into_owned();
            let context_embed = context_embed.unwrap().into_owned();
            let dp = context_embed.dot(&focus_embed);
            let sigmoid = 1f32 / (1f32 + (dp * (-1f32)).exp());

            println!("EBS of {} {}: {}", focus_word, context_word, sigmoid);
        }
    }

    Ok(())
}
