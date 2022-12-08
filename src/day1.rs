use sortby::*;

pub fn day1a(data: String) -> i32 {
    get_calories_iter(&data).max_by(|a, b| a.cmp(b)).unwrap()
}

pub fn day1b(data: String) -> i32 {
    get_calories_iter(&data).sort_by_desc(|i| *i).take(3).sum::<i32>()
}

fn get_calories_iter(data: &str) -> impl Iterator<Item = i32> + '_ {
    data.split("\n\n").map(|s| {
        s.split('\n')
            .map(|s| s.parse::<i32>().unwrap())
            .sum::<i32>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::read_file;

    #[test]
    fn day1a_test() {
        let data = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_owned();
        let result = day1a(data);
        assert_eq!(24000, result, "expected: <24000>, actual: <{}>", result);
    }

    #[test]
    fn day1b_test() {
        let data = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_owned();
        let result = day1b(data);
        assert_eq!(45000, result, "expected: <45000>, actual: <{}>", result);
    }

    #[test]
    #[ignore]
    fn day1a_actual() {
        let result = day1a(read_file("./data/day1.txt"));
        println!("day1a result: {}", result);
    }

    #[test]
    #[ignore]
    fn day1b_actual() {
        let result = day1b(read_file("./data/day1.txt"));
        println!("day1b result: {}", result);
    }
}
