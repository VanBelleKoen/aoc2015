#!/usr/bin/env bash

# Master setup script for a new Advent of Code day
# Combines creating the template and downloading input
# Usage: ./scripts/setup_day.sh <day_number>

if [ -z "$1" ]; then
    echo "Usage: ./scripts/setup_day.sh <day_number>"
    exit 1
fi

DAY=$1

echo "🎄 Setting up Advent of Code 2015 - Day $DAY 🎄"
echo ""

# Create the day template
echo "Step 1: Creating day template..."
./scripts/new_day.sh "$DAY"

echo ""
echo "Step 2: Downloading input..."
./scripts/download_input.sh "$DAY" 2015

echo ""
echo "✨ Setup complete! ✨"
echo ""
echo "Next steps:"
echo "  1. Implement your solution in src/days/day$(printf "%02d" $DAY).rs"
echo "  2. Run with: cargo run -- --day $DAY"
echo "  3. Test with: cargo test day$(printf "%02d" $DAY)"
