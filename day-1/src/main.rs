use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;

type Error = Box<dyn std::error::Error + Send + Sync>;

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
        .collect::<Result<Vec<u64>, ParseIntError>>()?;

    Ok(numbers)
}

fn count_larger_than_previous(numbers: Vec<u64>) -> u64 {
    let (res, _) = numbers.into_iter().fold((0, None), |(acc, prev), curr| {
        let acc = match (prev, curr) {
            (Some(prev), curr) if curr > prev => acc + 1,
            _ => acc,
        };
        (acc, Some(curr))
    });

    res
}

fn part_one() -> Result<u64, Error> {
    let numbers = parse_number_file("part-one.txt")?;

    Ok(count_larger_than_previous(numbers))
}

fn part_two() -> Result<u64, Error> {
    let numbers = parse_number_file("part-two.txt")?;

    let mut sliding_numbers = Vec::with_capacity(numbers.len());
    for n in 0..numbers.len() - 1 {
        let first = numbers.get(n);
        let second = numbers.get(n + 1);
        let third = numbers.get(n + 2);

        let (first, second, third) = match (first, second, third) {
            (Some(first), Some(second), Some(third)) => (first, second, third),
            _ => break,
        };

        sliding_numbers.push(first + second + third);
    }

    Ok(count_larger_than_previous(sliding_numbers))
}

fn main() -> Result<(), Error> {
    let res_one = part_one()?;
    println!("Part One result {}", res_one);

    let res_two = part_two()?;
    println!("Part Two result {}", res_two);

    Ok(())
}
