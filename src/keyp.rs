use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use std::collections::HashMap;

extern crate regex;

use regex::Regex;

pub fn get_threshold(dict: &HashMap<String, (f32, f32)>, word_c: &HashMap<String, i32>, percentage: usize) -> f32 {

    let mut frecs = Vec::new();

    for word in word_c.keys() {
        let (_, frec) = *(dict.get(&word.to_string()).unwrap_or(&(0.0,0.0)));
        frecs.push(frec);
    }
    frecs.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let perc = frecs.len() * percentage / 100;
    frecs[perc]
}

pub fn write_result(
    text: &str, 
    dict: &HashMap<String, (f32, f32)>, 
    (repl, reg): (&Regex, &Regex), 
    threshold: f32) -> std::io::Result<()>{

    let out = File::create("text.frec")?;
    let mut out = BufWriter::new(out);

    println!("Writing result Keyphraseness");
    for word in text.split(' ') {
        if !repl.is_match(word) {
            let trimmed_word = reg.replace_all(word, "");
            let trimmed_word = trimmed_word.to_lowercase();
            let (_, frec) = dict.get(&trimmed_word.to_string()).unwrap_or(&(0.0,0.0));
            if frec > &threshold && !trimmed_word.is_empty() {
                out.write_all(format!(" [[{}]]", word).as_bytes())?;
                //print!(" {} ({})", word, frec);
            } else {
                out.write_all(format!(" {}", word).as_bytes())?;
                //print!(" {} ({})", word, frec);
            }
        } else {
            out.write_all(word.as_bytes())?;
            //print!("{}", word);
        }
    }

    Ok(())
}