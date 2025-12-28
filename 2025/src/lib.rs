pub mod day1;
pub mod day2;
pub mod day3;
pub mod utility;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day1::solution::solve_1_a;
    use crate::day1::solution::solve_1_b;
    use crate::day2::solution::solve_2_a;
    use crate::day2::solution::solve_2_b;
    use crate::day3::solution::solve_3_a;
    use crate::day3::solution::solve_3_b;

    //#[test]
    fn day_1() {
        let result = solve_1_a("./input/1_test.txt".to_string());
        assert_eq!(result, 3);
        let result = solve_1_a("./input/1_a.txt".to_string());
        println!("Day1 A: {}", result);
        let result = solve_1_b("./input/1_test.txt".to_string());
        assert_eq!(result, 6);
        let result = solve_1_b("./input/1_b.txt".to_string());
        println!("Day1 B: {}", result);
    }
    //#[test]
    fn day_2() {
        let result = solve_2_a("./input/2_test.txt".to_string());
        assert_eq!(result, 1227775554);
        let result = solve_2_a("./input/2_a.txt".to_string());
        println!("Day2 A: {}", result);
        let result = solve_2_b("./input/2_test.txt".to_string());
        assert_eq!(result, 4174379265);
        let result = solve_2_b("./input/2_a.txt".to_string());
        println!("Day2 B: {}", result);
    }
    #[test]
    fn day_3() {
        let result = solve_3_a("./input/3_test.txt".to_string());
        assert_eq!(result, 357);
        let result = solve_3_a("./input/3_a.txt".to_string());
        println!("Day3 A: {}", result);
        let result = solve_3_b("./input/3_test.txt".to_string());
        assert_eq!(result, String::from("3121910778619"));
        let result = solve_3_b("./input/3_a.txt".to_string());
        println!("Day3 B: {}", result);
    }
}
