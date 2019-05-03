use std::env;
use std::process;

mod config;
use config::Config;

mod file;
use markovchain::Markov;

fn main() {
  // Initial program setup
  let config = Config::new(env::args()).unwrap_or_else(|err| {
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

  let markov = Markov::new(config.chain_size, &file_content);

  println!(
    "Creating paragraph with {} characters",
    config.sentence_size
  );

  let sentence = markov.create_sentences(config.sentence_size);
  println!("{}", sentence);
}

fn print_help() {
  println!("usage: markovchain file_path chain_size sentence_size(char)")
}
