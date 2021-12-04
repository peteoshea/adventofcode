use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[allow(dead_code)]
pub fn file_to_vec(filename: &str) -> Result<Vec<String>> {
  let file_in = File::open(filename.to_string())?;
  let file_reader = BufReader::new(file_in);
  Ok(file_reader.lines().filter_map(Result::ok).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_to_vec_returns_lines() {
        let filename = "./data/test/input.txt";
        let lines = file_to_vec(filename).unwrap();
        assert_eq!(lines, vec!["1", "2", "3", "4"]);
    }
}
