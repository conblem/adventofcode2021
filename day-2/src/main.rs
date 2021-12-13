use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

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

fn main() -> Result<(), Error> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let input = File::open(format!("{}/part-one.txt", manifest_dir))?;

    let lines = BufReader::new(input)
        .lines()
        .collect::<io::Result<Vec<String>>>()?;

    let commands = lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|input| input.parse())
        .collect::<Result<Vec<_>, _>>()?;

    let mut coordinates = Coordinate::default();
    for command in commands {
        coordinates.command(command);
    }

    let res_one = coordinates.multiply();
    println!("Part One result {}", res_one);

    //let res_two = part_two()?;
    //println!("Part Two result {}", res_two);

    Ok(())
}
