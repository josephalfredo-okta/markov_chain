use std::env;
use std::process;

mod config;
use config::Config;

mod file;
mod lib;
use lib::Markov;

fn main() {
  // Initial program setup
  let args: Vec<String> = env::args().skip(1).collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Config Error - {}", err);
    print_help();
    process::exit(1);
  });

  // Load file input
  let file_content = file::parse_file(&config.file_name).unwrap_or_else(|err| {
    eprintln!("File parse Error - {}", err);
    print_help();
    process::exit(1);
  });

  let markov = Markov::new(&file_content);

  let sentence = markov.create_sentences(config.sentence_size);
  println!("{}", sentence);
}

fn print_help() {
  println!("usage: markovchain file_path sentence_size(char)")
}
