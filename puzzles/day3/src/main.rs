use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug)]
struct DiagnosticInterpreter {
    gamma_rate: usize,
    epsilon_rate: usize,
}

impl DiagnosticInterpreter {
    fn get_power_consumption(&self) -> usize {
        self.epsilon_rate * self.gamma_rate
    }

    fn get_commonness_table(
        diagnostic_reader: &mut Lines<BufReader<File>>,
        length: usize,
    ) -> Vec<i32> {
        diagnostic_reader.fold(
            // Commonness table
            vec![0_i32; length],
            |mut acc, diagnostic_line| {
                if let Ok(diagnostic) = diagnostic_line {
                    diagnostic
                        .chars()
                        .enumerate()
                        .for_each(|(i, digit)| match digit {
                            '0' => {
                                acc[i] -= 1;
                            }
                            '1' => {
                                acc[i] += 1;
                            }
                            _ => panic!("Malformed data, submarine is corrupt!"),
                        })
                }
                acc
            },
        )
    }
}

impl From<Vec<i32>> for DiagnosticInterpreter {
    fn from(commonness_table: Vec<i32>) -> Self {
        let (gamma_rate, epsilon_rate) = commonness_table.iter().enumerate().fold(
            (0_usize, 0_usize),
            |mut acc, (i, table_item)| {
                dbg!(table_item);
                if *table_item > 0 {
                    acc.0 += 1;
                } else {
                    acc.1 += 1;
                }
                acc.0 = acc.0 << 1;
                acc.1 = acc.1 << 1;
                acc
            },
        );

        DiagnosticInterpreter {
            gamma_rate: gamma_rate >> 1,
            epsilon_rate: epsilon_rate >> 1,
        }
    }
}

fn main() {
    // Please run `cargo run` in `puzzles/day2`
    let navigation = File::open("src/input").unwrap();
    let mut diagnostic_reader = BufReader::new(navigation).lines();

    let commonness_table = DiagnosticInterpreter::get_commonness_table(&mut diagnostic_reader, 12);
    let diagnostic_interpreter = DiagnosticInterpreter::from(commonness_table);
    assert_eq!(diagnostic_interpreter.get_power_consumption(), 1131506);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_with_submarine_manual() {
        let navigation = File::open("src/test_input").unwrap();
        let mut diagnostic_reader = BufReader::new(navigation).lines();

        let commonness_table =
            DiagnosticInterpreter::get_commonness_table(&mut diagnostic_reader, 5);
        let diagnostic_interpreter = DiagnosticInterpreter::from(commonness_table);
        assert_eq!(diagnostic_interpreter.get_power_consumption(), 198);
    }
}
