use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

pub fn parse_dict(filename: &str) -> HashMap<String, (f32, f32)> {
    let mut res = HashMap::new();

    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    for line_op in file.lines() {
        let line = line_op.unwrap();
        let sp = line.split(';').collect::<Vec<&str>>();
        res.insert(sp[0].to_string(), (parse_input!(sp[1], f32), parse_input!(sp[2], f32)));
    }

    let pronouns = vec!["\"", "", "i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "your", "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "her", "hers", "herself", "it", "its", "itself", "they", "them", "their", "theirs", "themselves", "what", "which", "who", "whom", "this", "that", "these", "those", "am", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "having", "do", "does", "did", "doing", "a", "an", "the", "and", "but", "if", "or", "because", "as", "until", "while", "of", "at", "by", "for", "with", "about", "against", "between", "into", "through", "during", "before", "after", "above", "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some", "such", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "should", "now"];
    for word in pronouns {
        res.remove(word);
    }

    res
}