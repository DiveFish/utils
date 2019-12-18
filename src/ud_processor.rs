use conllx::Token;

pub fn reattach_punct(sentence: &mut Vec<Token>) {

    let mut prev_tok_head = 0;
    for i in 0..sentence.len() {

        let mut token = &mut sentence[i];
        if token.head_rel().expect("No head") == "punct" {
            if i == 0 {
                token.set_head(Some(i+2));
            } else if prev_tok_head == i+1 {
                token.set_head(Some(i+2));
            } else {
                token.set_head(Some(i));
            }
        }
        prev_tok_head = token.head().expect("No head");
    }
}