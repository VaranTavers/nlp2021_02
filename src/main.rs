extern crate xml;
extern crate regex;

use std::io::Write;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use std::collections::{HashMap};

use regex::Regex;

use xml::reader::{EventReader, XmlEvent};

struct Settings {
    links1_regex: Regex,
    links2_regex: Regex,
    links3_regex: Regex,
    links4_regex: Regex,
    links5_regex: Regex,
    links6_regex: Regex,
    char_filter_regex: Regex,
    de_ref: Regex,
    quotes: Regex,
    punctuation: Regex,

}

impl Default for Settings {
    
    fn default() -> Self {
        Settings {
            // { }
            links1_regex: Regex::new("\\{[^}]*\\}\\}").unwrap(),
            // [ tergerege ]
            links2_regex: Regex::new("[^\\[]\\[[^\\[\\]]+\\]").unwrap(),
            // < and > in html form
            links3_regex: Regex::new("&lt;[^&]*&gt;").unwrap(),
            links4_regex: Regex::new("<[^>]*>").unwrap(),
            links5_regex: Regex::new("http://[a-zA-Z0-9\\./-]+").unwrap(),
            links6_regex: Regex::new("\\[\\[[a-zA-Z ]+:[^\\]]+\\]\\]").unwrap(),
            char_filter_regex: Regex::new("[^a-zA-Z\\-\\., '\\[\\]]").unwrap(),
            de_ref: Regex::new("[\\[\\]]+").unwrap(),
            quotes: Regex::new("(''+)").unwrap(),
            punctuation: Regex::new("[,.;!|?/\n]").unwrap(),
        }

    }
}

fn filter_text(text: &str, settings: &Settings) -> String {
    if text.contains("abraham") && text.contains("image") {
        let out = File::create("abraham.txt").unwrap();
        let mut writer = BufWriter::new(out);
        writer.write_all(text.as_bytes()).unwrap();
    }
    let better_text = text.replace("\n", " ");
    let better_text = settings.links1_regex.replace_all(&better_text, " ");
    let better_text = settings.links2_regex.replace_all(&better_text, " ");
    let better_text = settings.links3_regex.replace_all(&better_text, " ");
    let better_text = settings.links4_regex.replace_all(&better_text, " ");
    let better_text = settings.links5_regex.replace_all(&better_text, " ");
    let better_text = settings.links6_regex.replace_all(&better_text, " ");
    let better_text = settings.punctuation.replace_all(&better_text, " ");
    let better_text = settings.char_filter_regex.replace_all(&better_text, "");
    let better_text = settings.quotes.replace_all(&better_text, " ");
    
    if text.contains("abraham") && text.contains("image") {
        let out = File::create("abraham2.txt").unwrap();
        let mut writer = BufWriter::new(out);
        writer.write_all(better_text.as_bytes()).unwrap();
    }

    better_text.to_lowercase().to_string()
}

fn do_word_count(text: &str, settings: &Settings) -> HashMap<String, usize> {
    let words = text.split(" ");
    let mut word_map = HashMap::new();

    for word in words {
        let word = &word.trim();

        if word.len() > 0 && word.len() < 30 {
            if word.contains("[[") || word.contains("]]") {
                //println!("EEEE");
                let entry = word_map.entry(settings.de_ref.replace_all(word, "").to_string()).or_insert(0);
                *entry = 1;
            } else {
                let _entry = word_map.entry(word.to_string()).or_insert(0);
            }
        }
    }

    word_map
}

fn main() {
    let file = File::open("enwiki-20061130-pages-articles.xml").unwrap();
    let file = BufReader::new(file);

    let settings = Settings::default();

    let mut huge_map: HashMap<String, (usize, usize)> = HashMap::new();
    let mut small_map: HashMap<String, usize> = HashMap::new();
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
                    for word in small_map.keys() {
                        let (a, b) = huge_map.entry(word.clone()).or_insert((0, 0));
                        *a += 1;
                        *b += small_map[word];
                    }
                    if number_of_documents % 1000 == 0 {
                        println!("No. {}", number_of_documents);
                    }
                    //if number_of_documents > 1000 {
                    //    break;
                    //}
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
        let (wc, nw_p) = huge_map[key];
        let val1 = (number_of_documents as f32 / wc as f32).log(2.0);
        let val2 = nw_p as f32 / wc as f32;

        writer.write_all(format!("{}; {}; {}\n", key, val1, val2).as_bytes()).unwrap();
    }
}