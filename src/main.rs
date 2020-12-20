use anyhow::Result;
use itertools::Itertools;
use std::io::{self, Write};
use std::iter::once;
use std::process;
use structopt::StructOpt;

use crate::args::{Arguments, Command};
use crate::words::WORDS_BY_LETTERS_USED;

mod args;
mod words;

trait App {
    fn run(&mut self) -> Result<()>;
}

fn main() {
    let mut app = make_app();
    if let Err(e) = app.run() {
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }
}

fn make_app() -> Box<dyn App> {
    let args = Arguments::from_args();

    match args.command {
        Command::Beehive { queen, workers } => Box::new(BeehiveApp::new(queen, workers)),
    }
}

struct BeehiveApp {
    queen: char,
    workers: String,
}

impl BeehiveApp {
    fn new(queen: char, workers: String) -> BeehiveApp {
        BeehiveApp { queen, workers }
    }

    /// Compute all k-combinations of the workers letters along with the queen.
    /// See: https://stackoverflow.com/a/29914908/4538652
    fn k_combinations(&self) -> Vec<String> {
        let mut combinations: Vec<String> = vec![];
        for k in 1..=self.workers.len() {
            let mut s = vec![0usize; k];
            
            if k <= self.workers.len() {
                for i in 0..s.len() {
                    s[i] = i;
                }
                let word = self.build_word(&s);
                if !combinations.contains(&word) {
                    combinations.push(word);
                }
                loop {
                    let mut i: isize = (k as isize) - 1;
                    while i >= 0 && s[i as usize] == self.workers.len() - k + (i as usize) {
                        i -= 1;
                    }
                    if i < 0 {
                        break;
                    }
                    s[i as usize] += 1;
                    i += 1;
                    while (i as usize) < k {
                        s[i as usize] = s[(i as usize) - 1] + 1;
                        i += 1;
                    }
                    let word = self.build_word(&s);
                    if !combinations.contains(&word) {
                        combinations.push(word);
                    }
                }
            }
        }
        combinations
    }

    /// Builds a word from the given list of indices. The returned word will
    /// include the queen letter, and the word will have its letters ordered.
    fn build_word(&self, subset: &Vec<usize>) -> String {
        let bytes = self.workers.as_bytes();
        once(self.queen)
            .chain(subset.iter().map(|i| bytes[*i] as char))
            .sorted()
            .dedup()
            .collect::<String>()
    }
}

impl App for BeehiveApp {
    fn run(&mut self) -> Result<()> {
        let answers = self.k_combinations().into_iter()
            .filter_map(|key| WORDS_BY_LETTERS_USED.get(&*key))
            .flat_map(|answers| answers.split_whitespace())
            .filter(|answer| answer.len() > 3)
            .map_into::<String>()
            .sorted()
            .dedup()
            .collect::<Vec<String>>();

        // Grab the stdout lock and write directly to it instead of using
        // println, which will be more performant if there are lots of words
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        for answer in answers {
            stdout.write_all(format!("{}\n", answer).as_ref())?;
        }
        Ok(())
    }
}
