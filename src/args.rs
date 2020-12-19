use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Arguments {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Solves the New York Times' Spelling Bee puzzle where they present 7 letters
    /// and playing involves inputting as many valid words that can be built from
    /// those letters, with some restrictions:
    ///  - Words must be at least 4 letters or more
    ///  - Words may contain the same letter multiple times
    ///  - Words must use the "golden" letter at least once
    ///  - There will be at least one answer which uses all 7 letters
    ///  See: https://www.nytimes.com/puzzles/spelling-bee
    Beehive {
        /// The single letter required to be in each word.
        queen: char,
       /// The six other letters that can be used to make words.
        workers: String,
    },
}
