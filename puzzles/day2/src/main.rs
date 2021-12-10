use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug)]
struct CruiseComputer {
    horizontal_position: usize,
    depth: usize,
    aim: usize,
}
impl CruiseComputer {
    fn get_planned_course(&self) -> usize {
        self.horizontal_position * self.depth
    }
    fn apply_course(&mut self, instruction_reader: Lines<BufReader<File>>) -> usize {
        instruction_reader.for_each(|navigation_command| {
            if let Ok(command) = navigation_command {
                // Interpret instructions
                match command.split(' ').enumerate().fold(
                    ("", 0_usize),
                    |mut interpreted_command, (i, item)| {
                        if i > 0 {
                            interpreted_command.1 = item.parse::<usize>().unwrap();
                        } else {
                            interpreted_command.0 = item;
                        }
                        interpreted_command
                    },
                ) {
                    // Apply instructions
                    ("down", amount) => {
                        self.aim += amount;
                    }
                    ("up", amount) => {
                        self.aim -= amount;
                    }
                    ("forward", amount) => {
                        self.horizontal_position += amount;
                        self.depth += (amount * self.aim);
                    }
                    _ => panic!("Submarine explodes! :)"),
                }
            }
        });
        // Provide planned course
        self.get_planned_course()
    }
}
impl Default for CruiseComputer {
    fn default() -> Self {
        Self {
            horizontal_position: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn main() {
    // Please run `cargo run` in `puzzles/day2`
    let navigation = File::open("src/input").unwrap();
    let instruction_reader = BufReader::new(navigation).lines();
    let mut cruise_computer = CruiseComputer::default();
    let planned_course = cruise_computer.apply_course(instruction_reader);

    // Answer
    assert_eq!(planned_course, 1340836560);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_with_submarine_manual() {
        let navigation = File::open("src/test_input").unwrap();
        let instruction_reader = BufReader::new(navigation).lines();
        let mut cruise_computer = CruiseComputer::default();
        let planned_course = cruise_computer.apply_course(instruction_reader);
        assert_eq!(planned_course, 900);
    }
}
