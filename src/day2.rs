fn day2a(data: String) -> i32 {
    -1
}

fn day2b(data: String) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn day2a_test() {
        let data = "A Y\nB X\nC Z".to_owned();
        let result = day2a(data);
        assert_eq!(15, result, "expected: <15>, actual: <{}>", result);
    }

    #[test]
    fn day2b_test() {
        let data = "A Y\nB X\nC Z".to_owned();
        let result = day2b(data);
        assert_eq!(0, result, "expected: <?>, actual: <{}>", result);
    }

    #[test]
    #[ignore]
    fn day2a_actual() {
        let result = day2a(read_file("./data/day2.txt"));
        println!("day1a result: {}", result);
    }

    #[test]
    #[ignore]
    fn day2b_actual() {
        let result = day2b(read_file("./data/day2.txt"));
        println!("day1b result: {}", result);
    }
}
