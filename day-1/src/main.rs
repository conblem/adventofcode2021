use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn parse_number_file(filename: &str) -> Result<Vec<u64>, Error> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let input = File::open(format!("{}/{}", manifest_dir, filename))?;

    let lines = BufReader::new(input)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;

    let numbers = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(numbers)
}

fn count_larger_than_previous(numbers: Vec<u64>) -> u64 {
    numbers
        .iter()
        .skip(1)
        .zip(numbers.iter())
        .map(|(curr, prev)| if curr > prev { 1 } else { 0 })
        .sum()
}

fn part_one() -> Result<u64, Error> {
    let numbers = parse_number_file("part-one.txt")?;

    Ok(count_larger_than_previous(numbers))
}

fn part_two() -> Result<u64, Error> {
    let numbers = parse_number_file("part-two.txt")?;

    let sliding_numbers = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .zip(numbers.iter().skip(2))
        .map(|((a, b), c)| [a, b, c].into_iter().sum())
        .collect();

    Ok(count_larger_than_previous(sliding_numbers))
}

fn main() -> Result<(), Error> {
    let res_one = part_one()?;
    println!("Part One result {}", res_one);

    let res_two = part_two()?;
    println!("Part Two result {}", res_two);

    Ok(())
}
