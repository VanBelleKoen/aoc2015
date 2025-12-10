use crate::days::Solution;

pub struct Day05;

fn is_nice_part1(s: &str) -> bool {
    let vowel_count = s
        .chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count();
    if vowel_count < 3 {
        return false;
    }

    let has_double = s.as_bytes().windows(2).any(|w| w[0] == w[1]);
    if !has_double {
        return false;
    }

    let forbidden = ["ab", "cd", "pq", "xy"];
    if forbidden.iter().any(|&f| s.contains(f)) {
        return false;
    }

    true
}

fn is_nice_part2(s: &str) -> bool {
    let has_pair = (0..s.len().saturating_sub(1)).any(|i| {
        let pair = &s[i..i + 2];
        s[i + 2..].contains(pair)
    });

    if !has_pair {
        return false;
    }

    let has_repeat_with_gap = s.as_bytes().windows(3).any(|w| w[0] == w[2]);
    if !has_repeat_with_gap {
        return false;
    }

    true
}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        let count = input.lines().filter(|line| is_nice_part1(line)).count();
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let count = input.lines().filter(|line| is_nice_part2(line)).count();
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let solution = Day05;
        assert_eq!(solution.part1("ugknbfddgicrmopn"), "1");
        assert_eq!(solution.part1("aaa"), "1");
        assert_eq!(solution.part1("jchzalrnumimnmhp"), "0");
        assert_eq!(solution.part1("haegwjzuvuyypxyu"), "0");
        assert_eq!(solution.part1("dvszwmarrgswjxmb"), "0");
        assert_eq!(
            solution.part1("ugknbfddgicrmopn\naaa\njchzalrnumimnmhp"),
            "2"
        );
    }

    #[test]
    fn test_part2() {
        let solution = Day05;
        assert_eq!(solution.part2("qjhvhtzxzqqjkmpb"), "1");
        assert_eq!(solution.part2("xxyxx"), "1");
        assert_eq!(solution.part2("uurcxstgmygtbstg"), "0");
        assert_eq!(solution.part2("ieodomkazucvgmuy"), "0");
    }
}
