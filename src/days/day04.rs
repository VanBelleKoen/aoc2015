use crate::days::Solution;

pub struct Day04;

use md5;

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        let key = input.trim();
        let mut n: u64 = 1;

        loop {
            let text = format!("{}{}", key, n);
            let digest = md5::compute(text);

            if digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0) == 0 {
                println!("found: {} -> {:x}", n, digest);
                return n.to_string();
            }

            n += 1;
        }
    }

    fn part2(&self, input: &str) -> String {
        let key = input.trim();
        let mut n: u64 = 1;

        loop {
            let text = format!("{}{}", key, n);
            let digest = md5::compute(text);

            if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
                println!("found: {} -> {:x}", n, digest);
                return n.to_string();
            }

            n += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let solution = Day04;
        assert_eq!(solution.part1("abcdef"), "609043");
        assert_eq!(solution.part1("pqrstuv"), "1048970");
    }

    #[test]
    fn test_part2() {
        // let solution = Day04;
    }
}
