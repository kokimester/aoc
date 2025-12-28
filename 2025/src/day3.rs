pub mod solution {
    use crate::utility::utility::open_and_read_file;

    fn find_max(line: Vec<i32>) -> (i32, usize) {
        let mut max: i32 = 0;
        let mut max_index: usize = 0;
        //search for the first lowest number
        for (i, val) in line.iter().enumerate() {
            if *val > max {
                max = *val;
                max_index = i;
            };
        }
        (max, max_index)
    }

    fn find_min(line: Vec<i32>) -> (i32, usize) {
        let mut min: i32 = 10;
        let mut min_index: usize = 0;
        //search for the first lowest number
        for (i, val) in line.iter().enumerate() {
            if *val < min {
                min = *val;
                min_index = i;
            };
        }
        (min, min_index)
    }
    /*
        Let:

    stack = empty

    to_remove = N - K

    For each digit d in s:

    While:

    stack is not empty

    top of stack < d

    to_remove > 0

    → pop stack
    → decrement to_remove

    Push d onto stack

    After processing all digits:

    If to_remove > 0, pop from the end of the stack

    Finally:

    Take the first K digits from the stack
        */

    fn calculate_digit_values(line: &String) -> Vec<i32> {
        let len = line.len() as i32;
        let mut values: Vec<i32> = vec![];
        for (i, val) in line.chars().enumerate() {
            let value_parsed: i32 = String::from(val).parse().expect("digit");
            let calculated_value: i32 = (len - i as i32) + 2 * (10 - value_parsed);
            values.push(calculated_value);
        }
        values
    }

    fn remove_smallest_digits(line: String) -> String {
        let mut number: String = line;
        while number.len() > 12 {
            let idx = find_max(calculate_digit_values(&number)).1;
            let removed_char = number.remove(idx);
        }
        number
    }

    fn get_highest_two_digit_number(line: String) -> i32 {
        let mut first_digit: i32 = 0;
        let mut last_digit: i32 = 0;
        let find_max = move |line: String| -> (i32, usize) {
            let mut max: i32 = 0;
            let mut max_index: usize = 0;
            //search for the highest number
            for (i, val) in line.chars().enumerate() {
                let value_parsed = String::from(val).parse().expect("digit");
                if value_parsed > max {
                    max = value_parsed;
                    max_index = i;
                };
            }
            (max, max_index)
        };
        let (max, max_index) = find_max(line.clone());
        //if the highest number is the last, set it as the second digit
        if max_index != (line.len() - 1) {
            first_digit = max;
            let line = line[max_index + 1..].to_string();
            let (max, _max_index) = find_max(line);
            last_digit = max;
        } else {
            last_digit = max;
            let line = line[0..max_index].to_string();
            let (max, _max_index) = find_max(line);
            first_digit = max;
        }
        first_digit * 10 + last_digit
    }

    fn add_numbers(mut a: String, mut b: String) -> String {
        let (alen, blen) = (a.len(), b.len());
        let prefix_len = if alen > blen {
            alen - blen
        } else {
            blen - alen
        };
        let prefix: String = String::from("0").repeat(prefix_len);
        if alen > blen {
            b = prefix + &b
        } else {
            a = prefix + &a
        };
        let mut carry: u8 = 0;
        let mut result: String = String::from("");
        for (ca, cb) in a.chars().rev().zip(b.chars().rev()) {
            let ca_number: u8 = ca.to_string().parse().expect("digit");
            let cb_number: u8 = cb.to_string().parse().expect("digit");
            let sum = ca_number + cb_number + carry;
            carry = sum / 10;
            let digit = (sum % 10).to_string();
            result.push_str(&digit[..]);
        }
        result = result.chars().rev().collect();
        result
    }

    pub fn solve_3_a(file: String) -> i32 {
        let mut sum: i32 = 0;
        match open_and_read_file(file) {
            Ok(lines) => {
                for (_index, line) in lines.iter().enumerate() {
                    let num = get_highest_two_digit_number(line.to_string());
                    sum += num;
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
        sum
    }

    pub fn solve_3_b(file: String) -> String {
        let mut sum: String = String::from("0").repeat(512);
        match open_and_read_file(file) {
            Ok(lines) => {
                for (_index, line) in lines.iter().enumerate() {
                    let number: String = remove_smallest_digits(line.to_string());
                    sum = add_numbers(sum, number);
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
        sum = sum.trim_start_matches('0').to_string();
        sum
    }
}
