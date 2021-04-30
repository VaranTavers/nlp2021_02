extern crate regex;

use std::path::Path;
use std::collections::HashMap;

use regex::Regex;

mod dict_creator;
mod dict_parser;
mod idf;
mod keyp;

use dict_creator::create_dict_file;


fn word_count(text: &str, reg: &Regex) -> HashMap<String, i32> {
    let text = text.to_lowercase();
    let text = reg.replace_all(&text, " ");
    let mut res = HashMap::new();

    println!("{}", text);
    for word in text.split(' ') {
        let entry = res.entry(word.to_string()).or_insert(0);
        *entry += 1;
    }

    res
}

fn main() {

    if !(Path::new("./idf.csv").exists()) {
        println!("The IDF file is missing. Rebuilding...");
        if !(Path::new("enwiki-20061130-pages-articles.xml").exists()) {
            println!("The Wiki file is missing, cannot rebuild!");
            return;
        }
        create_dict_file();
        println!("The IDF file has been rebuilt");
    }

    println!("Reading from file.");
    let text = std::fs::read_to_string("text.txt").unwrap();
    println!("Reading IDF file.");
    let dict = dict_parser::parse_dict("./idf.csv");    

    let reg = Regex::new("[^a-zA-Z\\-']+").unwrap();
    let repl = Regex::new("([.,;:?!\n\r\\)\\()])").unwrap();
    println!("Calculating TF.");
    let word_c = word_count(&text, &reg);
    let text = repl.replace_all(&text, " ${0}");

    let mut values = word_c.values().cloned().collect::<Vec<i32>>();
    let sum: i32 = values.iter().sum();
    values.sort_unstable();
    for i in values {
        println!("{} - {}", i, i as f32 / sum as f32);
    }

    let idf_threshold = idf::get_threshold(&dict, &word_c, 1);
    idf::write_result(&text, &dict, &word_c, (&repl, &reg), idf_threshold).unwrap();

    let keyp_threshold = keyp::get_threshold(&dict, &word_c, 6);
    keyp::write_result(&text, &dict, (&repl, &reg), keyp_threshold).unwrap();

    println!("{} - {}", idf_threshold, keyp_threshold);
}