struct Report {
    levels: Vec<u8>,
}

impl Report {
    fn from(levels: Vec<u8>) -> Self {
        Report { levels }
    }

    fn is_safe(&self) -> bool {
        if self.levels.len() < 2 {
            return true;
        }
        let mut diffs = self.levels.windows(2).map(|w| w[1] as i16 - w[0] as i16);
        let first = match diffs.next() {
            Some(0) => return false,
            Some(d) => d,
            None => return true,
        };
        let direction = first.signum();
        if direction == 0 {
            return false;
        }
        if (first.abs() < 1) || (first.abs() > 3) {
            return false;
        }
        for diff in diffs {
            if diff == 0 || diff.signum() != direction || !(1..=3).contains(&diff.abs()) {
                return false;
            }
        }
        true
    }

    fn is_safe_by_one(&self) -> bool {
        let n = self.levels.len();
        if n < 3 {
            return true;
        }
        if self.is_safe() {
            return true;
        }
        for i in 0..n {
            let mut reduced = self.levels.clone();
            reduced.remove(i);
            let report = Report::from(reduced);
            if report.is_safe() {
                return true;
            }
        }
        false
    }
}

fn parse_line(line: &str) -> Report {
    let levels: Vec<u8> = line
        .split_whitespace()
        .filter_map(|s| s.parse::<u8>().ok())
        .collect();
    Report::from(levels)
}

/// Parameters for solving Part 1 of the puzzle.
pub struct Part1Parameters {
    pub input_data: &'static str,
}

/// Solves Part 1 of the puzzle
///
/// # Arguments
///
/// * `params` - Parameters for the solver
///
/// # Returns
///
/// The solution as a string
pub fn solve_part1(params: Part1Parameters) -> String {
    let value: u32 = params
        .input_data
        .trim()
        .lines()
        .filter_map(|line| {
            let report = parse_line(line.trim());
            if report.is_safe() { Some(1) } else { None }
        })
        .sum();
    value.to_string()
}

/// Parameters for solving Part 2 of the puzzle.
pub struct Part2Parameters {
    pub input_data: &'static str,
}

/// Solves Part 2 of the puzzle
///
/// # Arguments
///
/// * `params` - Parameters for the solver
///
/// # Returns
///
/// The solution as a string
pub fn solve_part2(params: Part2Parameters) -> String {
    let value: u32 = params
        .input_data
        .trim()
        .lines()
        .filter_map(|line| {
            let report = parse_line(line.trim());
            if report.is_safe_by_one() {
                Some(1)
            } else {
                None
            }
        })
        .sum();
    value.to_string()
}
