use std::error::Error;
use std::fs;

/// Parse a file as String, removed new line
pub fn parse_file(filepath: &str) -> Result<(String), Box<dyn Error>> {
  println!("Loading ... {} ", filepath);

  let file_content = fs::read_to_string(filepath)?;

  let file_content = file_content.replace("\r", " ");
  let file_content = file_content.replace("\n", " ");
  Ok(file_content)
}
