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

    /// Compute all combinations of letters from the queen + workers.
    /// Using the "Shifting Algorithm" from
    /// https://hmkcode.com/calculate-find-all-possible-combinations-of-an-array-using-java/
    fn key_combinations(&self) -> Vec<String> {
        let mut combinations: Vec<String> = vec![];
        for k in 0..self.workers.len() {
            let mut ignore = vec![0usize; self.workers.len() - k];
            for w in 0..ignore.capacity() {
                ignore[w] = self.workers.len() - k + (w + 1);
            }

            let mut combination = vec![0usize; k];

            let mut i = 0;
            let mut r = 0;
            let mut g = 0;

            let mut terminate = false;
            while !terminate {
                while i < self.workers.len() && r < k {
                    if i != ignore[g] {
                        combination[r] = i;
                        r += 1;
                        i += 1;
                    } else {
                        if g != ignore.len() - 1 {
                            g += 1;
                        }
                        i += 1;
                    }
                }
                // Add new combination based on indices in combination
                let mut this_combination: String = String::new();
                for ri in 0..r {
                    this_combination.push(self.workers.as_bytes()[combination[ri]] as char);
                }
                let this_combination = once(self.queen).chain(this_combination.chars()).sorted().collect::<String>();
                combinations.push(this_combination);

                i = 0;
                r = 0;
                g = 0;
                terminate = true;

                for w in 0..ignore.len() {
                    if ignore[w] > w {
                        ignore[w] -= 1;
                        if w > 0 {
                            ignore[w-1] = ignore[w]-1;
                        }
                        terminate = false;
                        break;
                    }
                }
            }
        }
        combinations
    }
}

impl App for BeehiveApp {
    fn run(&mut self) -> Result<()> {
        let answers = self.key_combinations().into_iter()
            .sorted()
            .dedup()
            .filter_map(|key| WORDS_BY_LETTERS_USED.get(&*key))
            .flat_map(|answers| answers.split_whitespace())
            .filter(|answer| answer.len() > 3)
            .map_into::<String>()
            .sorted()
            .dedup()
            .collect::<Vec<String>>();

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        for answer in answers {
            stdout.write_all(format!("{}\n", answer).as_ref())?;
        }
        Ok(())
    }
}
