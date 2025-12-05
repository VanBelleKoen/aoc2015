#![allow(dead_code)]

/// Apply a direction character to coordinates
/// Returns the new (x, y) after moving
pub fn apply_direction(x: i32, y: i32, direction: char) -> (i32, i32) {
    match direction {
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        _ => (x, y),
    }
}

/// Parse a direction character and update mutable coordinates
pub fn move_in_direction(x: &mut i32, y: &mut i32, direction: char) {
    match direction {
        '^' => *y += 1,
        'v' => *y -= 1,
        '>' => *x += 1,
        '<' => *x -= 1,
        _ => {}
    }
}
