extern crate clap;
extern crate rust2vec;
extern crate stdinout;
extern crate utils;

use std::fs::File;
use std::io::BufWriter;
use stdinout::OrExit;

use clap::{App, Arg};

use utils::{
    bin_to_fifu, cmp_embeds, create_splits, load_w2v_embeddings, n_most_sim_embeds, read_w2v,
    read_w2v_vocab,
};

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

    //f64_list2file(&list, matches.value_of("OUTPUT_FILE").expect("File not found"));

    let input_dir = matches
        .value_of("INPUT_DIR")
        .expect("Could not read input directory");
    let output_dir = matches
        .value_of("OUTPUT_DIR")
        .expect("Could not read output directory");

    // Remember to set the output directories correctly in the method itself!
    //let list = create_splits(input_dir, &[2, 2, 6]);

    /*
    let focus_words = vec![
        "isst".to_string(),
        "isst".to_string(),
        "trinkt".to_string(),
        "trinkt".to_string(),
        "weiß".to_string(),
        "weiß".to_string(),
        "isst".to_string(),
        "isst".to_string(),
        "trinkt".to_string(),
        "trinkt".to_string(),
        "weiß".to_string(),
        "weiß".to_string(),
        "führte".to_string(),
        "führte".to_string(),
        "erstatteten".to_string(),
        "erstatteten".to_string(),
        "erstatteten".to_string(),
        "erstatteten".to_string(),
        "wollte".to_string(),
        "wollte".to_string(),
        "wollte".to_string(),
        "wollte".to_string(),
        "tragen".to_string(),
        "tragen".to_string(),
        "tragen".to_string(),
        "tragen".to_string(),
    ];
    let context_words = vec![
        "Regular_SUBJ_sie".to_string(),
        "Regular_OBJA_sie".to_string(),
        "Regular_SUBJ_Mann".to_string(),
        "Regular_OBJA_Mann".to_string(),
        "Regular_SUBJ_Computer".to_string(),
        "Regular_OBJA_Computer".to_string(),
        "Regular_SUBJ_Spaghetti".to_string(),
        "Regular_OBJA_Spaghetti".to_string(),
        "Regular_SUBJ_Milch".to_string(),
        "Regular_OBJA_Milch".to_string(),
        "Regular_SUBJ_alles".to_string(),
        "Regular_OBJA_alles".to_string(),
        "Regular_SUBJ_Gespräch".to_string(),
        "Regular_OBJA_Gespräch".to_string(),
        "Regular_SUBJ_Angeklagten".to_string(),
        "Regular_OBJA_Angeklagten".to_string(),
        "Regular_SUBJ_Strafanzeige".to_string(),
        "Regular_OBJA_Strafanzeige".to_string(),
        "Regular_SUBJ_niemand".to_string(),
        "Regular_OBJA_niemand".to_string(),
        "Regular_SUBJ_Krempel".to_string(),
        "Regular_OBJA_Krempel".to_string(),
        "Regular_SUBJ_Studierenden".to_string(),
        "Regular_OBJA_Studierenden".to_string(),
        "Regular_SUBJ_Risiko".to_string(),
        "Regular_OBJA_Risiko".to_string(),
    ];

    cmp_embeds(focus_words, context_words, input_dir, output_dir)
        .or_exit("Could not retrieve most similar words", 1);
    //n_most_sim_embeds("isst", 10,input_dir, output_dir).or_exit("Could not retrieve most similar words", 1);

    //n_most_sim_embeds("isst", 5, input_dir);
    */
    n_most_sim_embeds("Post", 20, input_dir, output_dir);
    n_most_sim_embeds("Post", 20, input_dir, input_dir);

    //let embeddings = load_w2v_embeddings(input_dir).or_exit("Cannot read from embeddings file", 1);
    //bin_to_fifu(output_dir, embeddings);
}
