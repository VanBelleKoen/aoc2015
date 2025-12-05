use crate::days::Solution;
use crate::utils::input;

pub struct Day02;

fn parse_dimensions(line: &str) -> (u32, u32, u32) {
    let parts: Vec<u32> = line.split('x').map(|s| s.parse().unwrap()).collect();
    (parts[0], parts[1], parts[2])
}

fn calculate_wrapping_paper(l: u32, w: u32, h: u32) -> u32 {
    let side1 = l * w;
    let side2 = w * h;
    let side3 = h * l;
    let smallest = side1.min(side2).min(side3);
    2 * side1 + 2 * side2 + 2 * side3 + smallest
}

fn calculate_ribbon(l: u32, w: u32, h: u32) -> u32 {
    let mut sides = [l, w, h];
    sides.sort_unstable();
    let wrap = 2 * (sides[0] + sides[1]);
    let bow = l * w * h;
    wrap + bow
}

impl Solution for Day02 {
    fn part1(&self, input_str: &str) -> String {
        let total: u32 = input::non_empty_lines(input_str)
            .iter()
            .map(|line| {
                let (l, w, h) = parse_dimensions(line);
                calculate_wrapping_paper(l, w, h)
            })
            .sum();
        total.to_string()
    }

    fn part2(&self, input_str: &str) -> String {
        let total: u32 = input::non_empty_lines(input_str)
            .iter()
            .map(|line| {
                let (l, w, h) = parse_dimensions(line);
                calculate_ribbon(l, w, h)
            })
            .sum();
        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let solution = Day02;
        assert_eq!(solution.part1("2x3x4"), "58");
        assert_eq!(solution.part1("1x1x10"), "43");
        assert_eq!(solution.part1("2x3x4\n1x1x10"), "101");
    }

    #[test]
    fn test_part2() {
        let solution = Day02;
        assert_eq!(solution.part2("2x3x4"), "34");
        assert_eq!(solution.part2("1x1x10"), "14");
        assert_eq!(solution.part2("2x3x4\n1x1x10"), "48");
    }
}
