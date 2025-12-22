pub mod day1;
pub mod utility;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day1::solution::solve_1;
    use crate::utility::utility::open_and_read_file;

    #[test]
    fn it_works() {
        let result = match open_and_read_file("./input/empty.txt".to_string()) {
            Ok(lines) => lines.len(),
            Err(e) => 1,
        };
        assert_eq!(result, 0);
        let result = solve_1("./input/empty.txt".to_string());
        assert_eq!(result, 0);
    }
}
