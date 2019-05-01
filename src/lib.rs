// extern crate test;

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

  pub fn create_sentences(&self, sentence_size: usize) -> String {
    let index = rand::thread_rng().gen_range(0, self.words.len() - 1);
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

        key_vec.remove(0);
        key_vec.push(new_word);
        key = key_vec.join(" ");
      } else {
        break;
      }
    }
    println!("Creating paragraph with {} characters", sentence_size);

    sentences
  }

  fn split_content(file_content: &String) -> Vec<&str> {
    file_content
      .split(" ")
      .filter(|word| !word.eq_ignore_ascii_case("") && !word.eq_ignore_ascii_case(" "))
      .collect()
  }

  fn create_chain(&mut self, chain_size: usize) {
    let n_words = self.words.len();
    let mut key_vec: Vec<&str> = Vec::new();
    for i in 0..self.chain_size {
      key_vec.push(self.words[i]);
    }
    let mut key = key_vec.join(" ");

    for i in 0..self.words.len() {
      if n_words > (i + chain_size) {
        let word = self.words[i + chain_size];
        if !self.chains.contains_key(&key) {
          self.chains.insert(key, vec![word]);
        } else {
          if let Some(pos) = self.chains.get_mut(&key) {
            pos.push(word);
          }
        }
        key_vec.remove(0);
        key_vec.push(word);
        key = key_vec.join(" ");
      }
    }
    // println!("{:?}", self.chains);
    println!("Chain size : {} distinct words pairs", self.chains.len());
  }

  #[cfg(test)]
  fn get_chain(&self) -> &HashMap<String, Vec<&'a str>> {
    &self.chains
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // use test::Bencher;

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

  // #[bench]
  // fn benchmark(bench: &mut Bencher) {
  //   bench.iter(|| {
  //     for i in 0..100 {
  //       let x = i + 1;
  //     }
  //   })
  // }
}
