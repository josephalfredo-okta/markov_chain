/// Store the program configuration / arguments
pub struct Config {
  pub file_name: String,
  pub chain_size: usize,
  pub sentence_size: usize,
}

impl Config {
  // Create new configuration from Args
  pub fn new(args: std::env::Args) -> Result<Config, String> {
    let mut args = args.skip(1);
    if args.len() < 3 {
      return Err(String::from("Not enough arguments"));
    }
    let file_name = args.next().expect("file_name");

    let chain_size = match args.next().expect("chain_size").parse() {
      Ok(v) => v,
      Err(e) => {
        return Err(format!("Error parsing arguments : {}", e));
      }
    };

    let sentence_size = match args.next().expect("sentence_size").parse() {
      Ok(v) => v,
      Err(e) => {
        return Err(format!("Error parsing arguments : {}", e));
      }
    };

    Ok(Config {
      file_name,
      chain_size,
      sentence_size,
    })
  }
}
