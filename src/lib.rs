#![feature(test)]
extern crate test;

use rand::Rng;
use std::collections::HashMap;

pub struct Markov<'a> {
  chain_size: usize,
  words: Vec<&'a str>,
  chains: HashMap<String, Vec<&'a str>>,
}

impl<'a> Markov<'a> {
  pub fn new(chain_size: usize, content: &'a String) -> Self {
    let words = Markov::split_content(&content);
    println!("Read : {} words", words.len());
    let chains = HashMap::new();
    let mut markov = Markov {
      chain_size,
      words,
      chains,
    };
    markov.create_chain(chain_size);
    markov
  }

  /// Create random sentences with length in chars
  pub fn create_sentences(&self, sentence_size: usize) -> String {
    let index = rand::thread_rng().gen_range(0, self.words.len() - self.chain_size);

    let mut key_vec: Vec<&str> = Vec::new();
    for i in index..(index + self.chain_size) {
      key_vec.push(self.words[i]);
    }
    let mut key = key_vec.join(" ");
    let mut sentences = key.clone();

    while sentences.len() < sentence_size as usize {
      if let Some(possible_words) = self.chains.get(&key) {
        let new_word = possible_words[rand::thread_rng().gen_range(0, possible_words.len())];
        sentences.push_str(" ");
        sentences.push_str(new_word);

        key_vec.remove(0); // this is not ideal ?
        key_vec.push(new_word);
        key = key_vec.join(" ");
      } else {
        break;
      }
    }
    sentences
  }

  /// Split String into Vec of &str
  fn split_content(file_content: &String) -> Vec<&str> {
    file_content
      .split(" ")
      .filter(|word| !word.eq_ignore_ascii_case("") && !word.eq_ignore_ascii_case(" "))
      .collect()
  }

  /// This will create markov chain matrix with n chain_size
  /// Currently pretty slow, mainly on creating the matrix
  fn create_chain(&mut self, chain_size: usize) {
    // sliding window with n size

    for i in 0..self.words.len() - chain_size {
      let key = (&self.words[i..(i + self.chain_size)]).join(" ");
      let word = self.words[i + chain_size];

      // let val: &mut Vec<&'a str> = self.chains.entry(key).or_insert(vec![word]){
      //   (*val).push(word);
      // }

      if !self.chains.contains_key(&key) {
        self.chains.insert(key, vec![word]);
      } else {
        if let Some(pos) = self.chains.get_mut(&key) {
          pos.push(word);
        }
      }
    }
    println!("{:#?}", self.chains);
    println!(
      "Created Chain size with {} distinct words pairs",
      self.chains.len()
    );
  }

  #[cfg(test)]
  fn get_chain(&self) -> &HashMap<String, Vec<&'a str>> {
    &self.chains
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // extern crate test;
  use test::Bencher;

  #[test]
  fn should_create_chain() {
    let content =
      String::from("the brown fox jumped over the lazy dog. brown fox to happy to eat.");
    let markov = Markov::new(2, &content);
    let chains = markov.get_chain();
    assert_eq!(chains.get(&String::from("the brown")), Some(&vec!["fox"]));
    assert_eq!(
      chains.get(&String::from("brown fox")),
      Some(&vec!["jumped", "to"])
    );
    assert_eq!(chains.get(&String::from("jumped over")), Some(&vec!["the"]));
  }

  #[bench]
  fn benchmark(bench: &mut Bencher) {
    bench.iter(|| should_create_chain())
  }
}
