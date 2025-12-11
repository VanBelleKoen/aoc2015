use crate::days::Solution;

pub struct Day06;

enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Command {
    instruction: Instruction,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn parse_command(line: &str) -> Command {
    let parts: Vec<&str> = line.split_whitespace().collect();

    let (instruction, start_idx) = if line.starts_with("turn on") {
        (Instruction::TurnOn, 2)
    } else if line.starts_with("turn off") {
        (Instruction::TurnOff, 2)
    } else {
        (Instruction::Toggle, 1)
    };

    let coords1: Vec<usize> = parts[start_idx]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let coords2: Vec<usize> = parts[start_idx + 2]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    Command {
        instruction,
        x1: coords1[0],
        y1: coords1[1],
        x2: coords2[0],
        y2: coords2[1],
    }
}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let mut grid = vec![vec![false; 1000]; 1000];

        for line in input.lines() {
            let cmd = parse_command(line);

            for x in cmd.x1..=cmd.x2 {
                for y in cmd.y1..=cmd.y2 {
                    match cmd.instruction {
                        Instruction::TurnOn => grid[x][y] = true,
                        Instruction::TurnOff => grid[x][y] = false,
                        Instruction::Toggle => grid[x][y] = !grid[x][y],
                    }
                }
            }
        }

        let count = grid.iter().flatten().filter(|&&light| light).count();
        count.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut grid = vec![vec![0i32; 1000]; 1000];

        for line in input.lines() {
            let cmd = parse_command(line);

            for x in cmd.x1..=cmd.x2 {
                for y in cmd.y1..=cmd.y2 {
                    match cmd.instruction {
                        Instruction::TurnOn => grid[x][y] += 1,
                        Instruction::TurnOff => grid[x][y] = (grid[x][y] - 1).max(0),
                        Instruction::Toggle => grid[x][y] += 2,
                    }
                }
            }
        }

        let total_brightness: i32 = grid.iter().flatten().sum();
        total_brightness.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let solution = Day06;
        assert_eq!(solution.part1("turn on 0,0 through 999,999"), "1000000");
        assert_eq!(solution.part1("toggle 0,0 through 999,0"), "1000");
        assert_eq!(
            solution.part1("turn on 0,0 through 999,999\nturn off 499,499 through 500,500"),
            "999996"
        );
    }

    #[test]
    fn test_part2() {
        let solution = Day06;
        assert_eq!(solution.part2("turn on 0,0 through 0,0"), "1");
        assert_eq!(solution.part2("toggle 0,0 through 999,999"), "2000000");
    }
}
