use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug)]
struct CruiseComputer {
    horizontal_position: usize,
    depth: usize,
}
impl CruiseComputer {
    fn get_planned_course(&self) -> usize {
        self.horizontal_position * self.depth
    }
    fn apply_course(&mut self, instruction_reader: Lines<BufReader<File>>) -> usize {
        instruction_reader.for_each(|navigation_command| {
            if let Ok(command) = navigation_command {
                match command
                    .split(' ')
                    .enumerate()
                    .fold(("", 0_usize), |mut acc, (i, item)| {
                        if i > 0 {
                            acc.1 = item.parse::<usize>().unwrap();
                        } else {
                            acc.0 = item;
                        }
                        acc
                    }) {
                    ("forward", amount) => {
                        self.horizontal_position += amount;
                    }
                    ("down", amount) => {
                        self.depth += amount;
                    }
                    ("up", amount) => {
                        self.depth -= amount;
                    }
                    _ => panic!("Submarine explodes! :)"),
                }
            }
        });
        self.get_planned_course()
    }
}
impl Default for CruiseComputer {
    fn default() -> Self {
        Self {
            horizontal_position: 0,
            depth: 0,
        }
    }
}

fn main() {
    // Please run `cargo run` in `puzzles/day2`
    let navigation = File::open("src/input").unwrap();
    let instruction_reader = BufReader::new(navigation).lines();
    let mut cruise_computer = CruiseComputer::default();
    let planned_course = cruise_computer.apply_course(instruction_reader);
    assert_eq!(planned_course, 1499229);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course() {
        let navigation = File::open("src/test_input").unwrap();
        let instruction_reader = BufReader::new(navigation).lines();
        let mut cruise_computer = CruiseComputer::default();
        let planned_course = cruise_computer.apply_course(instruction_reader);
        assert_eq!(planned_course, 150);
    }
}
