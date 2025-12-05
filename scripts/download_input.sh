#!/usr/bin/env bash

# Script to download Advent of Code input
# Requires AOC_SESSION in .env file or environment variable
# Usage: ./scripts/download_input.sh <day_number> [year]

# Load .env file if it exists
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | grep -v '^[[:space:]]*$' | xargs)
fi

if [ -z "$AOC_SESSION" ]; then
    echo "Error: AOC_SESSION not found"
    echo ""
    echo "Please create a .env file with your session cookie:"
    echo "  cp .env.example .env"
    echo "  # Edit .env and add your session cookie"
    echo ""
    echo "Or set it as an environment variable:"
    echo "  export AOC_SESSION=your_session_cookie_here"
    echo ""
    echo "Get your session cookie from adventofcode.com:"
    echo "  1. Log in to https://adventofcode.com"
    echo "  2. Open DevTools (F12)"
    echo "  3. Go to Application/Storage → Cookies"
    echo "  4. Copy the value of the 'session' cookie"
    exit 1
fi

if [ -z "$1" ]; then
    echo "Usage: ./scripts/download_input.sh <day_number> [year]"
    exit 1
fi

DAY=$1
YEAR=${2:-2015}
DAY_NUM=$(printf "%d" $DAY)
DAY_PADDED=$(printf "%02d" $DAY)
INPUT_FILE="inputs/day${DAY_PADDED}.txt"

if [ -f "$INPUT_FILE" ] && [ -s "$INPUT_FILE" ]; then
    echo "Input file $INPUT_FILE already exists and is not empty"
    read -p "Do you want to download it again? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
fi

echo "Downloading input for day $DAY_NUM, year $YEAR..."

HTTP_CODE=$(curl -s -w "%{http_code}" \
    -o "$INPUT_FILE" \
    -H "Cookie: session=$AOC_SESSION" \
    "https://adventofcode.com/$YEAR/day/$DAY_NUM/input")

if [ "$HTTP_CODE" = "200" ]; then
    echo "✓ Successfully downloaded input to $INPUT_FILE"
else
    echo "✗ Failed to download input (HTTP $HTTP_CODE)"
    echo "Check your AOC_SESSION token and that the day is available"
    rm -f "$INPUT_FILE"
    exit 1
fi
