use common::{Error, InputFileReader};

fn count_larger_than_previous(numbers: Vec<u64>) -> u64 {
    numbers
        .iter()
        .skip(1)
        .zip(numbers.iter())
        .map(|(curr, prev)| if curr > prev { 1 } else { 0 })
        .sum()
}

fn part_one(input_file_reader: &InputFileReader) -> Result<u64, Error> {
    let numbers = input_file_reader.read("part-one.txt")?;

    Ok(count_larger_than_previous(numbers))
}

fn part_two(input_file_reader: &InputFileReader) -> Result<u64, Error> {
    let numbers = input_file_reader.read("part-two.txt")?;

    let sliding_numbers = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .zip(numbers.iter().skip(2))
        .map(|((a, b), c)| [a, b, c].into_iter().sum())
        .collect();

    Ok(count_larger_than_previous(sliding_numbers))
}

fn main() -> Result<(), Error> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let input_file_reader = InputFileReader::new(manifest_dir);

    let res_one = part_one(&input_file_reader)?;
    println!("Part One result {}", res_one);

    let res_two = part_two(&input_file_reader)?;
    println!("Part Two result {}", res_two);

    Ok(())
}
