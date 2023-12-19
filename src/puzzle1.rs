use aoc2023::read_lines;

struct CalibrationValue {
    value: String,
}

struct CalibrationResult {
    values: Vec<CalibrationValue>,
}

impl CalibrationValue {
    fn replace(value: &str) -> i32 {
        match value {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => 0,
        }
    }

    fn extract(&self) -> Option<i32> {
        let needles = [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];
        let mut findings: Vec<(&str, usize)> = vec![];

        for needle in needles {
            let first = &self.value.find(needle);
            let last = &self.value.rfind(needle);

            if let (Some(first), Some(last)) = (first, last) {
                findings.push((needle, first.to_owned()));
                findings.push((needle, last.to_owned()));
            }
        }

        findings.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        if let (Some((first, _)), Some((last, _))) = (findings.first(), findings.last()) {
            let first = Self::replace(first);
            let last = Self::replace(last);

            let n: Option<i32> = format!("{}{}", first, last).parse().ok();

            n
        } else {
            None
        }
    }
}

impl CalibrationResult {
    fn run(&self) {
        let mut sum = 0;

        for value in &self.values {
            if let Some(value) = value.extract() {
                sum += value;
            }
        }

        println!("Puzzle 1 - {}", sum);
    }
}

impl Default for CalibrationResult {
    fn default() -> CalibrationResult {
        CalibrationResult { values: vec![] }
    }
}

impl From<String> for CalibrationValue {
    fn from(value: String) -> Self {
        CalibrationValue { value }
    }
}

pub fn run() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input/01.txt") {
        let mut res = CalibrationResult::default();
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(input) = line {
                res.values.push(CalibrationValue::from(input.to_owned()));
            }
        }

        res.run();
    } else {
        println!("No file found");
    }
}
