pub mod solution {
    use crate::utility::utility::open_and_read_file;
    pub fn solve_1(file: String) -> i32 {
        let mut safe_level_count: i32 = 0;
        match open_and_read_file(file) {
            Ok(lines) => {
                for (index, line) in lines.iter().enumerate() {
                    println!("Line {}: {}", index + 1, line);
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
        safe_level_count
    }
}
