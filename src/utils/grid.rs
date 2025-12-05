#![allow(dead_code)]

use std::fmt;

/// Directions for grid navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    /// Get all cardinal directions (N, S, E, W)
    pub fn cardinal() -> [Direction; 4] {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    /// Get all directions including diagonals
    pub fn all() -> [Direction; 8] {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ]
    }

    /// Get the offset for this direction (row, col)
    pub fn offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
            Direction::NorthEast => (-1, 1),
            Direction::NorthWest => (-1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (1, -1),
        }
    }
}

/// A position in a 2D grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        Position { row, col }
    }

    /// Move in a direction
    pub fn move_in(&self, direction: Direction) -> Position {
        let (dr, dc) = direction.offset();
        Position {
            row: self.row + dr,
            col: self.col + dc,
        }
    }

    /// Get all neighboring positions (cardinal directions)
    pub fn neighbors(&self) -> Vec<Position> {
        Direction::cardinal()
            .iter()
            .map(|&dir| self.move_in(dir))
            .collect()
    }

    /// Get all neighboring positions including diagonals
    pub fn neighbors_with_diagonals(&self) -> Vec<Position> {
        Direction::all()
            .iter()
            .map(|&dir| self.move_in(dir))
            .collect()
    }

    /// Manhattan distance to another position
    pub fn manhattan_distance(&self, other: &Position) -> i32 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

/// A 2D grid structure
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Grid<T> {
    /// Create a new grid from a 2D vector
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Grid { data, rows, cols }
    }

    /// Get value at position if valid
    pub fn get(&self, pos: Position) -> Option<&T> {
        if pos.row >= 0 && pos.col >= 0 {
            let row = pos.row as usize;
            let col = pos.col as usize;
            if row < self.rows && col < self.cols {
                return Some(&self.data[row][col]);
            }
        }
        None
    }

    /// Get mutable reference at position if valid
    pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        if pos.row >= 0 && pos.col >= 0 {
            let row = pos.row as usize;
            let col = pos.col as usize;
            if row < self.rows && col < self.cols {
                return Some(&mut self.data[row][col]);
            }
        }
        None
    }

    /// Check if position is within bounds
    pub fn is_valid(&self, pos: Position) -> bool {
        pos.row >= 0
            && pos.col >= 0
            && (pos.row as usize) < self.rows
            && (pos.col as usize) < self.cols
    }

    /// Find all positions matching a predicate
    pub fn find_all<F>(&self, predicate: F) -> Vec<Position>
    where
        F: Fn(&T) -> bool,
    {
        let mut positions = Vec::new();
        for (row, line) in self.data.iter().enumerate() {
            for (col, value) in line.iter().enumerate() {
                if predicate(value) {
                    positions.push(Position::new(row as i32, col as i32));
                }
            }
        }
        positions
    }
}
