use std::convert::TryFrom;
use std::str::FromStr;

use common::{Error, InputFileReader};

// this could definitely be replaced with a cargo dependency but where's the fun in that?
struct BitArray<const N: usize> {
    inner: [bool; N],
}

impl<const N: usize> TryFrom<BitArray<N>> for u64 {
    type Error = Error;

    fn try_from(value: BitArray<N>) -> Result<Self, Self::Error> {
        let mut res: u64 = 0;
        for bit in value.inner.into_iter() {
            let temp = res
                .checked_shl(1)
                .and_then(|res| res.checked_add(bit as u64));

            match temp {
                Some(temp) => res = temp,
                None => return Err("Overflow".into()),
            }
        }

        Ok(res)
    }
}

impl<const N: usize> FromStr for BitArray<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bit_array = Self { inner: [false; N] };
        let mut chars = s.chars();

        for item in &mut bit_array.inner {
            *item = match chars.next() {
                None => return Err(format!("String is not long enough, {}", s).into()),
                Some('0') => false,
                Some('1') => true,
                Some(char) => return Err(format!("Invalid character: {} in {}", char, s).into()),
            };
        }

        if chars.next().is_some() {
            return Err(format!("String is too long, {}", s).into());
        }

        Ok(bit_array)
    }
}

fn part_one(input_file_reader: &InputFileReader) -> Result<(), Error> {
    let bits: Vec<BitArray<12>> = input_file_reader.read("part-one.txt")?;

    Ok(())
}

fn main() -> Result<(), Error> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let input_file_reader = InputFileReader::new(manifest_dir);

    let res_one = part_one(&input_file_reader)?;
    println!("Part One result {:?}", res_one);

    /*let res_two = part_two(&input_file_reader)?;
    println!("Part Two result {}", res_two);*/

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    // todo: add error tests
    #[test]
    fn bit_array_works() -> Result<(), Error> {
        // is 22 in decimal
        let bit_array = BitArray::<5>::from_str("10110")?;
        assert_eq!(bit_array.inner, [true, false, true, true, false]);

        let number: u64 = bit_array.try_into()?;
        assert_eq!(number, 22);

        Ok(())
    }
}
