use crate::days::Solution;
use crate::utils::navigation;
use std::collections::HashSet;

pub struct Day03;

impl Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        let mut visited = HashSet::new();
        let mut x = 0;
        let mut y = 0;

        visited.insert((x, y));

        for c in input.chars() {
            navigation::move_in_direction(&mut x, &mut y, c);
            visited.insert((x, y));
        }

        visited.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut visited = HashSet::new();
        let mut santa_x = 0;
        let mut santa_y = 0;
        let mut robo_x = 0;
        let mut robo_y = 0;

        visited.insert((0, 0));

        for (i, c) in input.chars().enumerate() {
            if i % 2 == 0 {
                navigation::move_in_direction(&mut santa_x, &mut santa_y, c);
                visited.insert((santa_x, santa_y));
            } else {
                navigation::move_in_direction(&mut robo_x, &mut robo_y, c);
                visited.insert((robo_x, robo_y));
            }
        }

        visited.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let solution = Day03;
        assert_eq!(solution.part1(">"), "2");
        assert_eq!(solution.part1("^>v<"), "4");
        assert_eq!(solution.part1("^v^v^v^v^v"), "2");
    }

    #[test]
    fn test_part2() {
        let solution = Day03;
        assert_eq!(solution.part2("^v"), "3");
        assert_eq!(solution.part2("^>v<"), "3");
        assert_eq!(solution.part2("^v^v^v^v^v"), "11");
    }
}
