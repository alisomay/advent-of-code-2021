use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::ControlFlow;

struct SonarRegistry {
    last_depth_measurement: Option<usize>,
    comparison_window: VecDeque<Option<usize>>,
    single_measurement_increments: usize,
    windowed_measurement_increments: usize,
}

impl SonarRegistry {
    fn build(self, measurement_reader: Lines<BufReader<File>>) -> SonarRegistry {
        measurement_reader.fold(self, |mut sonar_registry, item| {
            if let Ok(value) = item {
                // Attempt a measurement
                // Assumes valid input
                let measurement: usize = value.trim().parse().unwrap();

                // Put measurement in the comparison window
                sonar_registry
                    .comparison_window
                    .push_back(Some(measurement));
                sonar_registry.comparison_window.pop_front();

                // Check registry
                if let Some(last_measurement) = sonar_registry.last_depth_measurement {
                    // Evaluate increment for single measurements
                    if measurement > last_measurement {
                        sonar_registry.single_measurement_increments += 1;
                    }

                    let window_sums: ControlFlow<_, (usize, usize)> = sonar_registry
                        .comparison_window
                        .iter()
                        .enumerate()
                        .try_fold((0, 0), |mut sums, (i, measurement)| {
                            if let Some(value) = measurement {
                                if i < 3 {
                                    sums.0 += *value;
                                }
                                if i > 0 && i < 4 {
                                    sums.1 += *value;
                                }
                                ControlFlow::Continue(sums)
                            } else {
                                ControlFlow::Break(())
                            }
                        });

                    // When the windows are ready to compare..
                    if let ControlFlow::Continue((last_window_sum, current_window_sum)) =
                        window_sums
                    {
                        if current_window_sum > last_window_sum {
                            sonar_registry.windowed_measurement_increments += 1;
                        }
                    }
                }

                // Record last measurement
                sonar_registry.last_depth_measurement = Some(measurement);
            }
            sonar_registry
        })
    }
}

impl Default for SonarRegistry {
    fn default() -> Self {
        Self {
            last_depth_measurement: None,
            comparison_window: VecDeque::from([None, None, None, None]),
            single_measurement_increments: 0,
            windowed_measurement_increments: 0,
        }
    }
}

fn main() {
    // Please run `cargo run` in `puzzles/day1`
    let measurements = File::open("src/input").unwrap();
    let measurement_reader = BufReader::new(measurements).lines();
    // This could have been tone with implementing `From` also.
    let sonar_registry = SonarRegistry::default().build(measurement_reader);

    // Answers
    assert_eq!(sonar_registry.single_measurement_increments, 1759);
    assert_eq!(sonar_registry.windowed_measurement_increments, 1805);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure_single() {
        let measurements = File::open("src/test_input").unwrap();
        let measurement_reader = BufReader::new(measurements).lines();
        let sonar_registry = SonarRegistry::default().build(measurement_reader);

        assert_eq!(sonar_registry.single_measurement_increments, 7);
    }
    #[test]
    fn measure_windowed() {
        let measurements = File::open("src/test_input").unwrap();
        let measurement_reader = BufReader::new(measurements).lines();
        let sonar_registry = SonarRegistry::default().build(measurement_reader);

        assert_eq!(sonar_registry.windowed_measurement_increments, 5);
    }
}
