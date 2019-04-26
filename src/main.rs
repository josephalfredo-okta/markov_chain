use std::env;
use std::error::Error;
use std::fs;
use std::process;

mod config;
use config::Config;

fn main() {
  let args: Vec<String> = env::args().skip(1).collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Config Error - {}", err);
    process::exit(1);
  });

  println!("Running Markov Chain ...");
  println!("Loading ... {} ", config.file_name);
  let file_content = parse_file(&config).unwrap_or_else(|err| {
    eprintln!("File parse Error - {}", err);
    process::exit(1);
  });
  println!("\nFile content : \n{}\n", file_content);
  println!("Creating paragraph with {} word(s)", config.sentence_size);
}

fn parse_file(config: &Config) -> Result<String, Box<dyn Error>> {
  let file_content = fs::read_to_string(&config.file_name)?;

  Ok(String::from(file_content))
}
