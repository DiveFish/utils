use test::stats::Summary;

pub fn spearman(x_list: &[f64], y_list: &[f64]) -> f64 {
    assert_eq!(x_list.len(), y_list.len());

    let x_mean = Summary::new(x_list).mean as f64;
    let y_mean = Summary::new(y_list).mean as f64;

    let mut numerator: f64 = 0.0;
    let mut denominator1: f64 = 0.0;
    let mut denominator2: f64 = 0.0;
    for i in 0..x_list.len() {
        numerator += (x_list[i] - x_mean) * (y_list[i] - y_mean);
        denominator1 += (x_list[i] - x_mean) as f64 * (x_list[i] - x_mean) as f64;
        denominator2 += (y_list[i] - y_mean) as f64 * (y_list[i] - y_mean) as f64;
    }

    numerator / (denominator1.sqrt() * denominator2.sqrt())
}

pub fn sent_cnt(file: &str) -> usize {
    let sents = ambiguity_stats::read_sentences(file);
    sents.len()
}

pub fn token_cnt(file: &str) -> usize {
    let sents = ambiguity_stats::read_sentences(file);
    let mut cnt = 0;
    for sent in sents {
        cnt += sent.len();
    }
    cnt
}
