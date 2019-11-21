use stdinout::OrExit;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use failure::{err_msg, Error};
use rust2vec_old;
use rust2vec_old::WriteWord2Vec;

use rust2vec::{
    embeddings::Embeddings, io::ReadEmbeddings, storage::StorageWrap,
    vocab::VocabWrap,
};

pub fn load_fifu(
    input_filename: &str,
) -> Result<Embeddings<VocabWrap, StorageWrap>, Error> {
    let file = File::open(input_filename).or_exit("Cannot read embeddings file", 1);
    let mut reader = BufReader::new(&file);
    let embeds: Embeddings<VocabWrap, StorageWrap> =
        Embeddings::read_embeddings(&mut reader).unwrap();

    Ok(
        embeds,
    )
}

pub fn write_fifu_to_w2v(embeddings: Embeddings<VocabWrap, StorageWrap>, output_dir: &str) -> Result<(), Error>
{
    let f = File::create(output_dir).or_exit("Cannot write to embeddings file", 1);
    let mut writer = BufWriter::new(f);

    let mut builder = rust2vec_old::Builder::new();

    for (word, embed) in embeddings.iter() {
        builder
            .push(word, embed.into_owned())
            .or_exit("Could not add embedding", 1);
    }

    let embeddings = builder.build().or_exit("Could not build embeddings", 1);
    embeddings
        .write_word2vec_binary_old(&mut writer)
        .or_exit("Could not write embedding file", 1);

    Ok(())
}