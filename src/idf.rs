use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use std::collections::HashMap;

extern crate regex;

use regex::Regex;

pub fn get_threshold(dict: &HashMap<String, (f32, f32)>, word_c: &HashMap<String, i32>, percentage: usize) -> f32 {

    let mut tf_idfs = Vec::new();
    for word in word_c.keys() {
        let (idf, _) = dict.get(&word.to_string()).unwrap_or(&(0.0,0.0));
        let tf_idf = *(word_c.get(&word.to_string()).unwrap_or(&0)) as f32 * idf;
        tf_idfs.push(tf_idf);
    }
    tf_idfs.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let perc = tf_idfs.len() * percentage / 100;

    tf_idfs[perc]
}

pub fn write_result(
    text: &str, 
    dict: &HashMap<String, (f32, f32)>, 
    word_c: &HashMap<String, i32>,
    (repl, reg): (&Regex, &Regex), 
    threshold: f32) -> std::io::Result<()>{

    let out = File::create("text.idf")?;
    let mut out = BufWriter::new(out);
    let s: i32 = word_c.values().sum();

    println!("Writing result IDF");
    for word in text.split(' ') {
        if !repl.is_match(word) {
            let trimmed_word = reg.replace_all(word, "");
            let trimmed_word = trimmed_word.to_lowercase();
            let (idf, _) = dict.get(&trimmed_word.to_string()).unwrap_or(&(0.0,0.0));
            let mut tf = *(word_c.get(&trimmed_word.to_string()).unwrap_or(&0));
            if tf as f32 / s as f32 > 0.01 {
                tf = 0;
            }
            let tf_idf = tf as f32 * idf;
            if tf_idf > threshold {
                out.write_all(format!(" [[{}]]", word).as_bytes())?;
                //print!(" {} ({}, {})", word, tf_idf, idf);
            } else {
                out.write_all(format!(" {}", word).as_bytes())?;
                //print!(" {} ({}, {})", word, tf_idf, idf);
            }
        } else {
            out.write_all(word.as_bytes())?;
            //print!("{}", word);
        }
    }
    Ok(())
}