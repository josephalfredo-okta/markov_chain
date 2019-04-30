use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::process;

mod config;
use config::Config;

mod file;
mod markov_chain;

fn main() {
  // Initial program setup
  let args: Vec<String> = env::args().skip(1).collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Config Error - {}", err);
    process::exit(1);
  });

  // Load file input
  let file_content = file::parse_file(&config.file_name).unwrap_or_else(|err| {
    eprintln!("File parse Error - {}", err);
    process::exit(1);
  });
  let words = split_content(&file_content);

  // Calculate Chains
  println!("Creating Markov Chain ...");

  // Create random fake news
  let markov_chain = create_chain(&words);

  let sentences = create_sentences(config.sentence_size, &words, markov_chain);
  println!("{}", sentences);
}

fn create_sentences(
  sentence_size: u32,
  words: &Vec<&str>,
  chains: HashMap<&str, Vec<&str>>,
) -> String {
  let mut first_word = words[rand::thread_rng().gen_range(0, words.len())];
  let mut sentences = String::from(first_word);

  while sentences.len() < sentence_size as usize {
    if let Some(possible_words) = chains.get(first_word) {
      let second_word = possible_words[rand::thread_rng().gen_range(0, possible_words.len())];
      sentences.push_str(" ");
      sentences.push_str(second_word);
      first_word = second_word;
    }
  }
  println!("Creating paragraph with {} word(s)", sentence_size);

  sentences
}

fn create_chain<'a>(words: &'a Vec<&str>) -> HashMap<&'a str, Vec<&'a str>> {
  let mut chains: HashMap<&str, Vec<&str>> = HashMap::new();
  let n_words = words.len();

  for (i, key) in words.iter().enumerate() {
    if n_words > (i + 1) {
      let word = words[i + 1];
      if !chains.contains_key(key) {
        chains.insert(key, vec![word]);
      } else {
        if let Some(pos) = chains.get_mut(key) {
          pos.push(word);
        }
      }
    }
  }
  println!("Chain size : {} distinct words", chains.len());
  chains
}

fn split_content(file_content: &String) -> Vec<&str> {
  let mut words: Vec<&str> = Vec::new();
  for word in file_content.split(" ") {
    if !word.eq_ignore_ascii_case("") && !word.eq_ignore_ascii_case(" ") {
      words.push(word);
    }
  }
  println!("Reading : {} words", words.len());
  words
}
