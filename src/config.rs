pub struct Config {
  pub file_name: String,
  pub sentence_size: u32,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, String> {
    if args.len() < 2 {
      return Err(String::from("Not enough arguments"));
    }

    let sentence_size = match args[1].clone().parse() {
      Ok(v) => v,
      Err(e) => {
        return Err(format!("Error parsing arguments : {}", e));
      }
    };

    Ok(Config {
      file_name: args[0].clone(),
      sentence_size,
    })
  }
}
