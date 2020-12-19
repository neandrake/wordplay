
use itertools::Itertools;

use phf_codegen::Map;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// This file was downloaded from
// https://github.com/dwyl/english-words/blob/master/words_alpha.txt
static WORDFILE: &str = "build/words_alpha.txt";

fn main() {
    println!("cargo:rerun-if-changed=./{}", WORDFILE);

    let file = File::open(WORDFILE).expect("Unable to load word file");
    let bufread = BufReader::new(file);
    let lines = bufread.lines();

    let mut phfmap: Map<String> = Map::new();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line_result in lines {
        let word: String = line_result.expect("Unable to read line");
        let key: String = word.chars().sorted().dedup().collect::<String>();
        let list: &mut Vec<String> = map.entry(key.clone()).or_insert(Vec::new());
        list.push(word);
    }

    for (key, list) in map {
        let words = list.iter().sorted().dedup().join(" ");
        let value = format!("\"{}\"", words);
        phfmap.entry(key, &value);
    }

    let out_path = Path::new("src/words.rs");
    let mut out_file = BufWriter::new(File::create(out_path).expect("Unable to open word-map file"));
    write!(&mut out_file,
        "//! This is an auto-generated file. Do not make modifications here.

#![allow(clippy::unreadable_literal)]

use phf::Map;

/// A map of words keyed by the letters that make up those words.
/// The letters in each key are sorted and unique, for example:
/// `ablno -> balloon`
/// The values of this map are a single string of words separated by
/// whitespace. This is due to the limitation that static maps cannot
/// use values of varying size at compile-time (i.e. `Vec`). The words
/// in keys appear in alphabetical order.
pub static WORDS_BY_LETTERS_USED: Map<&'static str, &'static str> = {};",
        phfmap.build())
        .expect("Error saving word-map file");
}
