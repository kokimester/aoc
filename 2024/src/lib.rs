use std::fs::File;
use std::io::{self, BufRead, BufReader};
fn open_and_read_file(file: String) -> Result<Vec<String>, io::Error> {
    // Open the file
    let file = File::open(file)?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Collect lines into a vector
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();

    lines
}

//rules:
// strictly monotonic increasing or decreasing
// min distance 1
// max distance 3
fn determine_level_safety_a(level: &String) -> bool {
    if level == "" {
        return false;
    }
    let numbers: Vec<_> = level
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();
    let differences: Vec<i32> = numbers
        .windows(2) // Create sliding windows of size 2
        .map(|pair| pair[1] - pair[0]) // Calculate the difference between neighboring elements
        .collect(); // Collect the results into a new vector
    if (differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0)) == false {
        return false;
    }
    if (differences.iter().all(|&x| x.abs() >= 1) && differences.iter().all(|&x| x.abs() <= 3))
        == false
    {
        return false;
    }

    true
}

fn determine_level_safety_b(level: &String) -> bool {
    if level == "" {
        return false;
    }
    // println!("{level}");
    let numbers: Vec<_> = level
        .split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    // iterate all possible removed numbers in mind
    for i in 0..numbers.len() {
        let new_vector: Vec<_> = numbers
            .iter()
            .enumerate()
            .filter(|&(index, _)| index != i) // Skip the element at the current index
            .map(|(_, &value)| value) // Extract the value
            .collect();

        // println!("After removing element at index {}: {:?}", i, new_vector);

        let differences: Vec<i32> = new_vector
            .windows(2) // Create sliding windows of size 2
            .map(|pair| pair[1] - pair[0]) // Calculate the difference between neighboring elements
            .collect(); // Collect the results into a new vector
        if (differences.iter().all(|&x| x > 0) || differences.iter().all(|&x| x < 0)) == false {
            continue;
        }
        if (differences.iter().all(|&x| x.abs() >= 1) && differences.iter().all(|&x| x.abs() <= 3))
            == false
        {
            continue;
        }
        return true;
    }
    false
}

pub fn solve_2_a(file: String) -> i32 {
    let mut safe_level_count: i32 = 0;
    match open_and_read_file(file) {
        Ok(lines) => {
            for (_index, line) in lines.iter().enumerate() {
                // println!("Line {}: {}", index + 1, line);
                if determine_level_safety_a(line) {
                    safe_level_count += 1;
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    safe_level_count
}

pub fn solve_2_b(file: String) -> i32 {
    let mut safe_level_count: i32 = 0;
    match open_and_read_file(file) {
        Ok(lines) => {
            for (_index, line) in lines.iter().enumerate() {
                // println!("Line {}: {}", index + 1, line);
                if determine_level_safety_b(line) {
                    safe_level_count += 1;
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    safe_level_count
}

use regex::Regex;
pub fn solve_3_a(file: String) -> i32 {
    match open_and_read_file(file) {
        Ok(lines) => {
            let mut sum: i32 = 0;
            for (_index, line) in lines.iter().enumerate() {
                // println!("Line {}: {}", _index + 1, line);
                // Define the regular expression for the format "mul(number,number)"
                let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

                // Find all matches and collect them into a vector
                let substrings: Vec<_> = re
                    .find_iter(line)
                    .map(|mat| mat.as_str().to_string())
                    .collect();
                // substrings.iter().for_each(|s| println!("{s}"));
                // Iterate over each substring and extract numbers
                // Regex to match individual numbers within the format "mul(number,number)"
                let number_re = Regex::new(r"\d+").unwrap();

                let pairs: Vec<_> = substrings
                    .iter()
                    .filter_map(|s| {
                        let numbers: Vec<i32> = number_re
                            .find_iter(s)
                            .filter_map(|mat| mat.as_str().parse::<i32>().ok())
                            .collect();

                        if numbers.len() == 2 {
                            // Return a tuple (first number, second number) if both are found
                            // println!("{} => {} {}", s, numbers[0], numbers[1]);
                            return Some((numbers[0], numbers[1]));
                        } else {
                            None // Ignore invalid cases
                        }
                    })
                    .collect();
                // pairs
                // .iter()
                // .for_each(|p| println!("{} * {} = {}", p.0, p.1, p.0 * p.1));
                let multiples: Vec<i32> = pairs.into_iter().map(|p| p.0 * p.1).collect();
                let line_sum: i32 = multiples.iter().sum();
                sum += line_sum;
                // println!("{}", sum);
                // let stripped_line = re.replace_all(line, "").to_string();
                // println!("{}", stripped_line);
            }
            return sum;
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    0
}
pub fn solve_3_b(file: String) -> i32 {
    match open_and_read_file(file) {
        Ok(lines) => {
            let mut sum: i32 = 0;
            let mut include_in_sum: bool = true;
            for (_index, line) in lines.iter().enumerate() {
                // println!("Line {}: {}", _index + 1, line);

                // Define regex for the three formats
                // Adjust these patterns to match your actual formats
                let pattern1 = r"do\(\)"; // Matches "abc" followed by 3 digits
                let pattern2 = r"don\'t\(\)"; // Matches 3 digits followed by "def"
                let pattern3 = r"mul\(\d{1,3},\d{1,3}\)"; // Matches "xyz" followed by 3 digits

                // Combine the patterns into a single regex
                let combined_pattern = format!("({})|({})|({})", pattern1, pattern2, pattern3);
                let regex = Regex::new(&combined_pattern).expect("Invalid regex");

                // Collect all matches in order of appearance
                let all_substrings: Vec<_> =
                    regex.find_iter(line).map(|mat| mat.as_str()).collect();

                // println!("{}", line);
                // println!("{:?}", all_substrings);
                let substrings: Vec<_> = all_substrings
                    .iter()
                    .filter_map(|s| match s {
                        &"do()" => {
                            include_in_sum = true;
                            None
                        }
                        &"don't()" => {
                            include_in_sum = false;
                            None
                        }
                        _ => {
                            if include_in_sum {
                                return Some(s);
                            }
                            None
                        }
                    })
                    .collect();

                // substrings.iter().for_each(|s| println!("{s}"));
                // Iterate over each substring and extract numbers
                // Regex to match individual numbers within the format "mul(number,number)"
                let number_re = Regex::new(r"\d+").unwrap();

                let pairs: Vec<_> = substrings
                    .iter()
                    .filter_map(|s| {
                        let numbers: Vec<i32> = number_re
                            .find_iter(s)
                            .filter_map(|mat| mat.as_str().parse::<i32>().ok())
                            .collect();

                        if numbers.len() == 2 {
                            // Return a tuple (first number, second number) if both are found
                            // println!("{} => {} {}", s, numbers[0], numbers[1]);
                            return Some((numbers[0], numbers[1]));
                        } else {
                            None // Ignore invalid cases
                        }
                    })
                    .collect();
                // pairs
                // .iter()
                // .for_each(|p| println!("{} * {} = {}", p.0, p.1, p.0 * p.1));
                let multiples: Vec<i32> = pairs.into_iter().map(|p| p.0 * p.1).collect();
                let line_sum: i32 = multiples.iter().sum();
                sum += line_sum;
                // println!("{}", sum);
                // let stripped_line = re.replace_all(line, "").to_string();
                // println!("{}", stripped_line);
            }
            return sum;
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_day2() {
        assert_eq!(solve_2_a("./src/2_test.txt".to_string()), 2);
        println!("{}", solve_2_a("./src/2.txt".to_string()));
        assert_eq!(solve_2_b("./src/2_test.txt".to_string()), 4);
        println!("{}", solve_2_b("./src/2.txt".to_string()));
    }
    #[test]
    fn test_day3() {
        assert_eq!(solve_3_a("./src/3_test.txt".to_string()), 161);
        println!("Day 3a: {}", solve_3_a("./src/3.txt".to_string()));
        assert_eq!(solve_3_b("./src/3_test_b.txt".to_string()), 48);
        println!("Day 3b: {}", solve_3_b("./src/3.txt".to_string()));
    }
}
