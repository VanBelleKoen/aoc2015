#!/usr/bin/env bash

# Day template for Advent of Code
# Usage: ./scripts/new_day.sh <day_number>

if [ -z "$1" ]; then
    echo "Usage: ./scripts/new_day.sh <day_number>"
    exit 1
fi

DAY=$(printf "%02d" $1)
DAY_FILE="src/days/day${DAY}.rs"
INPUT_FILE="inputs/day${DAY}.txt"
EXAMPLE_FILE="examples/day${DAY}.txt"

# Create the day file from template
if [ -f "$DAY_FILE" ]; then
    echo "Day $DAY already exists at $DAY_FILE"
else
    cat > "$DAY_FILE" << 'EOF'
use crate::days::Solution;

pub struct DayXX;

impl Solution for DayXX {
    fn part1(&self, input: &str) -> String {
        // TODO: Implement part 1
        "Not implemented".to_string()
    }

    fn part2(&self, input: &str) -> String {
        // TODO: Implement part 2
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        let solution = DayXX;
        assert_eq!(solution.part1(input), "expected");
    }

    #[test]
    fn test_part2() {
        let input = "";
        let solution = DayXX;
        assert_eq!(solution.part2(input), "expected");
    }
}
EOF
    # Replace XX with the day number
    sed -i '' "s/XX/$DAY/g" "$DAY_FILE"
    echo "Created $DAY_FILE"
fi

# Create empty input file
touch "$INPUT_FILE"
echo "Created $INPUT_FILE"

# Add the module to days/mod.rs if not already present
MOD_LINE="pub mod day${DAY};"
if ! grep -q "$MOD_LINE" src/days/mod.rs; then
    # Insert before the trait definition
    sed -i '' "/pub trait Solution/i\\
$MOD_LINE
" src/days/mod.rs
    echo "Added module to src/days/mod.rs"
fi

# Add the day to main.rs solver list
MATCH_LINE="        $1 => Box::new(days::day${DAY}::Day${DAY}),"
if ! grep -q "days::day${DAY}::Day${DAY}" src/main.rs 2>/dev/null; then
    echo ""
    echo "Don't forget to add this day to the match statement in main.rs:"
    echo "    $MATCH_LINE"
fi

echo ""
echo "Day $DAY setup complete! 🎄"
echo "Files created:"
echo "  - $DAY_FILE"
echo "  - $INPUT_FILE"
