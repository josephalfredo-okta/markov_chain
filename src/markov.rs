use rand::Rng;
use std::collections::HashMap;

pub struct Markov<'a> {
  words: Vec<&'a str>,
  chains: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Markov<'a> {
  pub fn new(content: &'a String) -> Self {
    let words = Markov::split_content(&content);
    let chains = HashMap::new();
    let mut markov = Markov { words, chains };
    markov.create_chain();
    markov
  }

  pub fn create_sentences(&self, sentence_size: u32) -> String {
    let mut first_word = self.words[rand::thread_rng().gen_range(0, self.words.len())];
    let mut sentences = String::from(first_word);

    while sentences.len() < sentence_size as usize {
      if let Some(possible_words) = self.chains.get(first_word) {
        let second_word = possible_words[rand::thread_rng().gen_range(0, possible_words.len())];
        sentences.push_str(" ");
        sentences.push_str(second_word);
        first_word = second_word;
      }
    }
    println!("Creating paragraph with {} word(s)", sentence_size);

    sentences
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

  fn create_chain(&mut self) {
    let n_words = self.words.len();

    for (i, key) in self.words.iter().enumerate() {
      if n_words > (i + 1) {
        let word = self.words[i + 1];
        if !self.chains.contains_key(key) {
          self.chains.insert(key, vec![word]);
        } else {
          if let Some(pos) = self.chains.get_mut(key) {
            pos.push(word);
          }
        }
      }
    }
    println!("Chain size : {} distinct words", self.chains.len());
  }
}
