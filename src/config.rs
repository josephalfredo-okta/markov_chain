pub struct Config {
  pub file_name: String,
  pub chain_size: usize,
  pub sentence_size: usize,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, String> {
    if args.len() < 3 {
      return Err(String::from("Not enough arguments"));
    }

    let chain_size = match args[1].clone().parse() {
      Ok(v) => {
        if v < 1 {
          return Err(format!("Chains size must be above 1"));
        } else {
          v
        }
      }
      Err(e) => {
        return Err(format!("Error parsing arguments : {}", e));
      }
    };

    let sentence_size = match args[2].clone().parse() {
      Ok(v) => v,
      Err(e) => {
        return Err(format!("Error parsing arguments : {}", e));
      }
    };

    Ok(Config {
      file_name: args[0].clone(),
      chain_size,
      sentence_size,
    })
  }
}
