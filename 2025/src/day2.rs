pub mod solution {
    use crate::utility::utility::open_and_read_file;

    fn parse_ranges(line: String) -> Vec<(i64, i64)> {
        let mut result: Vec<(i64, i64)> = vec![];
        /*let ranges =*/
        line.split(",").for_each(|x| {
            let mut bounds = x.split("-");
            let pairs = std::iter::from_fn(move || {
                let a = bounds.next()?;
                let b = bounds.next()?;
                Some((a, b))
            });
            for pair in pairs {
                let int_pair: (i64, i64) = (
                    pair.0.parse().expect("number"),
                    pair.1.parse().expect("number"),
                );
                result.push(int_pair);
            }
        });
        result
    }

    fn is_double_sequence(number: String) -> bool {
        let (first, second) = number.split_at(number.len() / 2);
        first == second
    }

    fn is_repeating_sequence(number: String) -> bool {
        let len = number.len();
        // println!("number is : {}", number);
        for i in 1..=len / 2 {
            let sequence: &str = &number[0..i];
            //skip numbers that cannot be a sequence
            if len % i != 0 {
                continue;
            }
            //build sequence
            let built_number: String = String::from(sequence).repeat(len / i);
            // println!("{} ?= {}", built_number, number);
            if built_number == number {
                // println!("{} == {}", built_number, number);
                return true;
            }
        }
        false
    }

    fn get_invalid_ids(range: (i64, i64), is_invalid_id_func: fn(String) -> bool) -> Vec<i64> {
        let mut result: Vec<i64> = vec![];
        for num in range.0..=range.1 {
            let num_str = num.to_string();
            if is_invalid_id_func(num_str) {
                result.push(num);
            };
        }
        result
    }

    pub fn solve_2_a(file: String) -> i64 {
        let mut invalid_ids: Vec<i64> = vec![];
        match open_and_read_file(file) {
            Ok(lines) => {
                for (_index, line) in lines.iter().enumerate() {
                    let ranges = parse_ranges(line.to_string());
                    for range in ranges {
                        // println!("{} - {}", range.0, range.1);
                        let ids = get_invalid_ids(range, is_double_sequence);
                        // println!("{:?}", ids);
                        if ids.is_empty() == false {
                            invalid_ids = [&invalid_ids[..], &ids[..]].concat();
                        }
                        // println!("{:?}", invalid_ids);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
        invalid_ids.iter().sum()
    }

    pub fn solve_2_b(file: String) -> i64 {
        let mut invalid_ids: Vec<i64> = vec![];
        match open_and_read_file(file) {
            Ok(lines) => {
                for (_index, line) in lines.iter().enumerate() {
                    let ranges = parse_ranges(line.to_string());
                    for range in ranges {
                        // println!("{} - {}", range.0, range.1);
                        let ids = get_invalid_ids(range, is_repeating_sequence);
                        // println!("{:?}", ids);
                        if ids.is_empty() == false {
                            invalid_ids = [&invalid_ids[..], &ids[..]].concat();
                        }
                        // println!("{:?}", invalid_ids);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
        invalid_ids.iter().sum()
    }
}
