use crate::days::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let floor = input.chars().fold(0, |floor, c| match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        });
        floor.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut floor = 0;
        for (i, c) in input.chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => {}
            }
            if floor == -1 {
                return (i + 1).to_string();
            }
        }
        "Never enters basement".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let solution = Day01;
        assert_eq!(solution.part1("(())"), "0");
        assert_eq!(solution.part1("()()"), "0");
        assert_eq!(solution.part1("((("), "3");
        assert_eq!(solution.part1("(()(()("), "3");
        assert_eq!(solution.part1("))((((("), "3");
        assert_eq!(solution.part1("())"), "-1");
        assert_eq!(solution.part1("))("), "-1");
        assert_eq!(solution.part1(")))"), "-3");
        assert_eq!(solution.part1(")())())"), "-3");
    }

    #[test]
    fn test_part2() {
        let solution = Day01;
        assert_eq!(solution.part2(")"), "1");
        assert_eq!(solution.part2("()())"), "5");
    }
}
