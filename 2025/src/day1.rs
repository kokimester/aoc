pub mod solution {
    use crate::utility::utility::open_and_read_file;

    fn turn_dial(current_pos: i32, input: String) -> (i32, i32) {
        let direction: i32 = if input.starts_with("L") { -1 } else { 1 };
        let mut pos: i32 = current_pos;
        let amount: i32 = match input[1..].parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Error({}) while parsing dial input {}", e, input);
                0
            }
        };
        let mut zero_crossing: i32 = if current_pos == 0 && direction == -1 {
            -1
        } else {
            0
        };
        pos = pos + (amount * direction);
        //pos = if pos < 0 { 100 + pos } else { pos };
        loop {
            if pos >= 0 && pos <= 100 {
                break;
            }
            if pos < 0 {
                pos = pos + 100;
                println!("Zero crossing: {} -> {} = {}", current_pos, input, pos);
                zero_crossing += 1;
            }
            if pos > 100 {
                pos = pos - 100;
                println!("Zero crossing: {} -> {} = {}", current_pos, input, pos);
                zero_crossing += 1;
            }
        }
        pos = pos % 100;
        (pos, zero_crossing)
    }

    pub fn solve_1_a(file: String) -> i32 {
        let mut dial_pos: i32 = 50;
        let mut zero_count: i32 = 0;
        match open_and_read_file(file) {
            Ok(lines) => {
                for (_index, line) in lines.iter().enumerate() {
                    // println!("Line {}: {}", index + 1, line);
                    let new_pos = turn_dial(dial_pos, line.to_string()).0;
                    println!("{} -> {} = {}", dial_pos, line, new_pos);
                    dial_pos = new_pos;
                    zero_count = zero_count + if dial_pos == 0 { 1 } else { 0 };
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
        zero_count
    }

    pub fn solve_1_b(file: String) -> i32 {
        let mut dial_pos: i32 = 50;
        let mut zero_count: i32 = 0;
        match open_and_read_file(file) {
            Ok(lines) => {
                for (_index, line) in lines.iter().enumerate() {
                    // println!("Line {}: {}", index + 1, line);
                    let (new_pos, zero_crossing) = turn_dial(dial_pos, line.to_string());
                    println!(
                        "{} -> {} = {} (crossing: {})",
                        dial_pos, line, new_pos, zero_crossing
                    );
                    dial_pos = new_pos;
                    zero_count = zero_count + if dial_pos == 0 { 1 } else { 0 };
                    zero_count = zero_count + zero_crossing;
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
        zero_count
    }
}
