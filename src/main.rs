extern crate xml;
extern crate regex;

use std::io::Write;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use std::collections::{HashMap, HashSet};

use regex::Regex;

use xml::reader::{EventReader, XmlEvent};

struct Settings {
    links1_regex: Regex,
    links2_regex: Regex,
    links3_regex: Regex,
    links4_regex: Regex,
    links5_regex: Regex,
    char_filter_regex: Regex,
    starting_spec: Regex,
    ending_spec: Regex,
    quotes: Regex,
    punctuation: Regex,

}

impl Default for Settings {
    
    fn default() -> Self {
        Settings {
            links1_regex: Regex::new("\\{\\{[^}]*\\}\\}").unwrap(),
            links2_regex: Regex::new("\\[[^\\]]+\\]").unwrap(),
            links3_regex: Regex::new("&lt;[^&]*&gt;").unwrap(),
            links4_regex: Regex::new("<[^>]*>").unwrap(),
            links5_regex: Regex::new("http://[a-zA-Z0-9\\./-]+").unwrap(),
            char_filter_regex: Regex::new("[^a-zA-Z\\-\\., ']").unwrap(),
            starting_spec: Regex::new("^[^a-zA-Z]*").unwrap(),
            ending_spec: Regex::new("[^a-zA-Z]*$").unwrap(),
            quotes: Regex::new("(''+)").unwrap(),
            punctuation: Regex::new("[,.;!?/\n]").unwrap(),
        }

    }
}

fn filter_text(text: &str, settings: &Settings) -> String {
    let better_text = settings.links1_regex.replace_all(&text, " ");
    let better_text = settings.links2_regex.replace_all(&better_text, " ");
    let better_text = settings.links3_regex.replace_all(&better_text, " ");
    let better_text = settings.links4_regex.replace_all(&better_text, " ");
    let better_text = settings.links5_regex.replace_all(&better_text, " ");
    let better_text = settings.punctuation.replace_all(&better_text, " ");
    let better_text = settings.char_filter_regex.replace_all(&better_text, "");
    let better_text = settings.quotes.replace_all(&better_text, " ");

    better_text.to_lowercase().to_string()
}

fn do_word_count(text: &str, settings: &Settings) -> HashSet<String> {
    let words = text.split(" ");
    let mut word_set = HashSet::new();

    for word in words {
        let word = settings.ending_spec.replace(&word.trim(), "").to_string();
        let word = settings.starting_spec.replace(&word, "");

        if word.len() > 0 && word.len() < 30 {
            word_set.insert(word.to_string());
        }
    }

    word_set
}

fn main() {
    let file = File::open("enwiki-20061130-pages-articles.xml").unwrap();
    let file = BufReader::new(file);

    let settings = Settings::default();

    let mut huge_map: HashMap<String, usize> = HashMap::new();
    let mut small_map: HashSet<String> = HashSet::new();
    let mut text_on = false;
    let mut worth_saving = false;

    let mut number_of_documents = 0;


    println!("Parsing files...");
    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "title" {
                    worth_saving = false;
                } else if name.local_name == "text" {
                    text_on = true;                    
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "page" && worth_saving {
                    number_of_documents += 1;
                    for word in &small_map {
                        let entry = huge_map.entry(word.clone()).or_insert(0);
                        *entry += 1;    
                    }
                    if number_of_documents % 1000 == 0 {
                        println!("No. {}", number_of_documents);
                    }
                    if number_of_documents > 10000 {
                        break;
                    }
                } else if name.local_name == "text" {
                    text_on = false;                    
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                if text_on {
                    if text.len() > 1024 {
                        worth_saving = true;
                        small_map = do_word_count(&filter_text(&text, &settings), &settings);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {
            }
        }
    }

    println!("The file has been parsed, writing idf.csv");
    let out = File::create("idf.csv").unwrap();
    let mut writer = BufWriter::new(out);
    for key in huge_map.keys() {
        let val = (number_of_documents as f32 / huge_map[key] as f32).log(2.0);

        writer.write_all(format!("{}; {}\n", key, val).as_bytes()).unwrap();
    }
}