use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error as StdError;
use std::fmt::Display;

pub type Error = Box<dyn StdError + Send + Sync + 'static>;

pub struct InputFileReader {
    manifest_dir: &'static str,
}

impl InputFileReader {
    pub fn new(manifest_dir: &'static str) -> Self {
        Self {
            manifest_dir
        }
    }

    pub fn read<T, F>(&self, filename: F) -> Result<Vec<T>, Error>
    where
        T: FromStr,
        Error: From<T::Err>,
        F: Display,
    {
        let input = File::open(format!("{}/{}", self.manifest_dir, filename))?;

        let lines = BufReader::new(input)
            .lines()
            .collect::<Result<Vec<_>, _>>()?;

        let parsed_lines = lines
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(parsed_lines)
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn should_read_file() -> Result<(), Error> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_file_reader = InputFileReader::new(manifest_dir);
        let test_file: Vec<u64> = input_file_reader.read("test.txt")?;

        assert_eq!(test_file, vec![1, 2, 3]);

        Ok(())
    }

    #[test]
    fn should_convert_to_box() {
        let test_err = io::Error::from(io::ErrorKind::NotFound);
        let _boxed_err: Error = test_err.into();
    }
}
