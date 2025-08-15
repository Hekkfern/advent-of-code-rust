# How to use this project

- [How to use this project](#how-to-use-this-project)
  - [Requirements](#requirements)
  - [Basic actions](#basic-actions)
    - [Manually](#manually)
      - [Execute a puzzle solution](#execute-a-puzzle-solution)
      - [Run the unit tests for one of the puzzles](#run-the-unit-tests-for-one-of-the-puzzles)
  - [Advanced actions](#advanced-actions)
    - [How to add a new puzzle to the project](#how-to-add-a-new-puzzle-to-the-project)
  - [How to extract puzzle instructions from the webpage](#how-to-extract-puzzle-instructions-from-the-webpage)

## Requirements

First, it is important that you prepare the necessary software in your machine in order to be able to use this project.

Read [SETUP_DEV_ENVIRONMENT.md](./SETUP_DEV_ENVIRONMENT.md) page to help you execute the preparation steps.

## Basic actions

### Manually

#### Execute a puzzle solution

```bash
cargo build --release -p aoc_<year>_<day> --part <part>
```

where:
* <year> is the year (format 20XX) of the puzzle.
* <day> is the day (from 1 to 25) of the puzzle, with leading zeros to make two digits.
* <part> is the part number to run, `1` for Part 1 and `2` for Part 2.

#### Run the unit tests for one of the puzzles

```bash
cargo test --release -p aoc_<year>_<day>
```

where:
* <year> is the year (format 20XX) of the puzzle.
* <day> is the day (from 1 to 25) of the puzzle, with leading zeros to make two digits.

## Advanced actions

### How to add a new puzzle to the project

Read the docs of the **aoc-assistant** internal tool in its [README.md](../tools/aoc-assistant/README.md) page.

## How to extract puzzle instructions from the webpage

Read the docs of the **aoc-assistant** internal tool in its [README.md](../tools/aoc-assistant/README.md) page.
