#![allow(dead_code)]

use std::fs;

/// Read input file for a specific day
pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}

/// Read example input file for a specific day
pub fn read_example(day: u8) -> String {
    let path = format!("examples/day{:02}.txt", day);
    fs::read_to_string(&path).unwrap_or_else(|_| String::new())
}

/// Parse input into lines
pub fn lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Parse input into non-empty lines
pub fn non_empty_lines(input: &str) -> Vec<&str> {
    input.lines().filter(|line| !line.is_empty()).collect()
}

/// Parse input into lines and map with a function
pub fn parse_lines<T, F>(input: &str, f: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    input.lines().map(f).collect()
}

/// Parse input as a grid of characters
pub fn parse_char_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Parse input as integers (one per line)
pub fn parse_integers(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().expect("Failed to parse integer"))
        .collect()
}

/// Split input by blank lines into groups
pub fn split_by_blank_lines(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

/// Parse comma-separated values
pub fn parse_csv<T>(input: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .trim()
        .split(',')
        .map(|s| s.trim().parse().expect("Failed to parse CSV value"))
        .collect()
}
