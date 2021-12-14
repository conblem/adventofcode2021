use std::str::FromStr;

use common::{Error, InputFileReader};

const FORWARD: &str = "forward";
const DOWN: &str = "down";
const UP: &str = "up";

#[derive(Clone, Copy)]
enum CommandType {
    Forward,
    Down,
    Up,
}

impl FromStr for CommandType {
    type Err = Error;

    fn from_str(command: &str) -> Result<Self, Self::Err> {
        match command {
            FORWARD => Ok(Self::Forward),
            DOWN => Ok(Self::Down),
            UP => Ok(Self::Up),
            _ => Err("Invalid command".into()),
        }
    }
}

#[derive(Clone, Copy)]
struct Command {
    command_type: CommandType,
    distance: u64,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (command_type, distance) = input
            .split_once(' ')
            .ok_or_else(|| Error::from("String not splitable"))?;

        let command_type = command_type.parse()?;
        let distance = distance.parse()?;

        Ok(Self {
            command_type,
            distance,
        })
    }
}

// y is reversed as the submarine travels down the y axis
#[derive(Default)]
struct Coordinate {
    x: u64,
    y: u64,
}

impl Coordinate {
    fn command(&mut self, command: Command) {
        let distance = command.distance;
        match command.command_type {
            CommandType::Forward => self.x += distance,
            CommandType::Down => self.y += distance,
            CommandType::Up => self.y -= distance,
        }
    }

    fn multiply(&self) -> u64 {
        self.x * self.y
    }
}

#[derive(Default)]
struct Aim {
    coordinates: Coordinate,
    aim: u64,
}

impl Aim {
    fn command(&mut self, command: Command) {
        let distance = command.distance;
        match command.command_type {
            // down and up just modify the aim
            CommandType::Down => self.aim += distance,
            CommandType::Up => self.aim -= distance,
            // forward has a new behaviour on top of the normal coordinate behaviour
            CommandType::Forward => {
                self.coordinates.command(command);
                let down = Command {
                    command_type: CommandType::Down,
                    distance: self.aim * distance,
                };
                self.coordinates.command(down);
            }
        }
    }

    fn multiply(&self) -> u64 {
        self.coordinates.multiply()
    }
}

fn part_one(input_file_reader: &InputFileReader) -> Result<u64, Error> {
    let commands = input_file_reader.read("part-one.txt")?;

    let mut coordinates = Coordinate::default();
    for command in commands {
        coordinates.command(command);
    }

    Ok(coordinates.multiply())
}

fn part_two(input_file_reader: &InputFileReader) -> Result<u64, Error> {
    let commands = input_file_reader.read("part-two.txt")?;

    let mut aim = Aim::default();
    for command in commands {
        aim.command(command);
    }

    Ok(aim.multiply())
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
