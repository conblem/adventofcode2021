use std::ops::{Index, Mul, Not};
use std::str::FromStr;

use common::{Error, InputFileReader};

const WIDTH: usize = 12;

// creates a bit pattern of 0..011111;
// to get this pattern we take 1 and shift it to the right by the WIDTH
// this results in a pattern of 0..0100000
// the zeros to the right correspond to the WIDTH
// now if we subtract 1 we get 0..0011111
// the ones to the right correspond to the WIDTH
const BIT_MASK: u64 = (1 << WIDTH) - 1;

#[derive(Clone, Copy)]
struct BinaryU64(u64);

impl Mul for BinaryU64 {
    type Output = u64;

    fn mul(self, rhs: BinaryU64) -> Self::Output {
        self.0 * rhs.0
    }
}

impl Not for BinaryU64 {
    type Output = BinaryU64;

    fn not(self) -> Self::Output {
        // the and pattern should be 0..011111
        // where the amount of 1's is the WIDTH
        // using and we mask num and keep zeros left of the width
        BinaryU64(!self.0 & BIT_MASK)
    }
}

impl Index<usize> for BinaryU64 {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if index > 63 {
            panic!("index out of bounds");
        }

        let res = self.0 & (1 << index) != 0;

        // creates a static reference to a bool
        if res {
            &true
        } else {
            &false
        }
    }
}

impl From<BinaryU64> for u64 {
    fn from(input: BinaryU64) -> Self {
        input.0
    }
}

impl FromStr for BinaryU64 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 2 is the radix for binary numbers
        let res = u64::from_str_radix(s, 2)?;
        Ok(Self(res))
    }
}

// this implementation is only generic for fun
// should probably just be implemented for i64 or something alike
impl<T> From<T> for BinaryU64
where
    T: IntoIterator,
    T::Item: Copy + Default + Ord,
{
    fn from(input: T) -> Self {
        let zero = T::Item::default();

        let mut res = 0;

        for value in input {
            let bit = value > zero;

            res *= 2;
            res += bit as u64;
        }

        Self(res)
    }
}

fn part_one(input_file_reader: &InputFileReader) -> Result<u64, Error> {
    let bits: Vec<BinaryU64> = input_file_reader.read("part-one.txt")?;

    let gamma: BinaryU64 = bits
        .into_iter()
        .fold([0; WIDTH], |mut acc, curr| {
            for i in 0..WIDTH {
                acc[i] += if curr[i] { 1 } else { -1 };
            }
            acc
        })
        .into_iter()
        .rev()
        .into();

    Ok(gamma * !gamma)
}

enum BitCriteria {
    Oxygen,
    CO2,
}

impl BitCriteria {
    fn is_met(&self, val: i64) -> bool {
        match self {
            BitCriteria::Oxygen => val >= 0,
            BitCriteria::CO2 => !(val >= 0),
        }
    }
}

impl BitCriteria {
    fn find(&self, mut bits: Vec<BinaryU64>) -> Result<BinaryU64, Error> {
        // we start with the first bit
        for i in 0..WIDTH {

            // we find out the distribution of the bits
            let criteria = bits
                .iter()
                .fold(0, |acc, curr| acc + if curr[i] { 1 } else { -1 });

            let criteria = self.is_met(criteria);

            // remove all the bits that don't match the criteria
            bits.retain(|bit| bit[i] == criteria);

            // we have found the last bit that matches the criteria
            if bits.len() == 1 {
                break;
            }
        }

        bits.pop().ok_or_else(|| Error::from("No bits left"))
    }
}

fn part_two(input_file_reader: &InputFileReader) -> Result<u64, Error> {
    let bits: Vec<BinaryU64> = input_file_reader.read("part-one.txt")?;

    let oxygen_rating = BitCriteria::Oxygen.find(bits.clone())?;
    let co2_rating = BitCriteria::CO2.find(bits)?;

    Ok(oxygen_rating * co2_rating)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_works() -> Result<(), Error> {
        let binary = BinaryU64::from_str("01")?;
        assert_eq!(binary.0, 1);

        let binary = BinaryU64::from_str("10")?;
        assert_eq!(binary.0, 2);

        let binary = BinaryU64::from_str("11")?;
        assert_eq!(binary.0, 3);

        Ok(())
    }

    #[test]
    fn index_works() -> Result<(), Error> {
        let binary = BinaryU64::from_str("10")?;
        assert_eq!(binary[0], false);
        assert_eq!(binary[1], true);

        Ok(())
    }

    #[test]
    fn is_met_works_oxygen() {
        // value is over 0 so we want to keep the true bits
        assert!(BitCriteria::Oxygen.is_met(5));

        // there are the same amount of values in the case of oxygen we want to keep the true bit
        assert!(BitCriteria::Oxygen.is_met(0));

        // value is under 0 so we want to keep the false bits
        assert!(!BitCriteria::Oxygen.is_met(-5));
    }

    #[test]
    fn is_met_works_co2() {
        // value is over 0 so we want to keep the false bits
        assert!(!BitCriteria::CO2.is_met(5));

        // there are the same amount of values in the case of co2 we want to keep the false bit
        assert!(!BitCriteria::CO2.is_met(0));

        // value is under 0 so we want to keep the true bits
        assert!(BitCriteria::CO2.is_met(-5));
    }
}
