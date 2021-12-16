use std::convert::TryFrom;
use std::ops::Index;
use std::str::FromStr;

use common::{Error, InputFileReader};

// this could definitely be replaced with a cargo dependency but where's the fun in that?
#[derive(Clone, Copy)]
struct BitArray<const N: usize> {
    inner: [bool; N],
}

impl<const N: usize> BitArray<N> {
    fn try_multiply(self, other: BitArray<N>) -> Result<u64, Error> {
        let this = u64::try_from(self)?;
        let other = u64::try_from(other)?;

        Ok(this * other)
    }

    // bitwise invert
    fn invert(&self) -> Self {
        let mut reversed = self.clone();
        for i in 0..N {
            reversed.inner[i] = !reversed.inner[i];
        }
        reversed
    }
}

impl<const N: usize> Index<usize> for BitArray<N> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

// this implementation is only generic for fun
// should probably just be implemented for i64 or something alike
impl<T, const N: usize> From<[T; N]> for BitArray<N>
where
    T: Default + Ord,
{
    fn from(input: [T; N]) -> Self {
        let mut res = Self { inner: [false; N] };
        for i in 0..N {
            // if input[i] is bigger than zero we count it as true
            // this works for signed and unsigned numbers
            res.inner[i] = input[i] > T::default();
        }
        res
    }
}

impl<const N: usize> TryFrom<BitArray<N>> for u64 {
    type Error = Error;

    fn try_from(value: BitArray<N>) -> Result<Self, Self::Error> {
        let mut res: u64 = 0;
        for bit in value.inner.into_iter() {
            let temp = res
                .checked_mul(2)
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

const PART_ONE_LENGTH: usize = 12;

fn part_one(input_file_reader: &InputFileReader) -> Result<u64, Error> {
    let bits: Vec<BitArray<PART_ONE_LENGTH>> = input_file_reader.read("part-one.txt")?;

    let res = bits
        .into_iter()
        .fold([0; PART_ONE_LENGTH], |mut acc, curr| {
            for i in 0..acc.len() {
                acc[i] += if curr[i] { 1 } else { -1 };
            }
            acc
        });

    let gamma = BitArray::from(res);
    let epsilon = gamma.invert();

    gamma.try_multiply(epsilon)
}

const PART_TWO_LENGTH: usize = 5;

fn part_two(input_file_reader: &InputFileReader) -> Result<(), Error> {
    let mut bits: Vec<BitArray<PART_TWO_LENGTH>> = input_file_reader.read("test.txt")?;

    for i in 0..PART_TWO_LENGTH {
        let criteria = bits
            .iter()
            .fold(0, |acc, curr| acc + if curr[i] { 1 } else { -1 });

        let criteria = criteria >= 0;

        bits.retain(|bit| bit[i] == criteria);
        if bits.len() == 1 {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let input_file_reader = InputFileReader::new(manifest_dir);

    let res_one = part_one(&input_file_reader)?;
    println!("Part One result {}", res_one);

    let res_two = part_two(&input_file_reader)?;
    /*println!("Part Two result {}", res_two);*/

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

    #[test]
    fn bit_array_overflow_works() {
        // 128 bit max number
        let bit_array = BitArray::from([1; 128]);
        let overflow_err = u64::try_from(bit_array);
        assert!(overflow_err.is_err());
    }
}
