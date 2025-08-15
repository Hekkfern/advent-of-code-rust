# aoc-assistant

`aoc-assistant` is a command-line tool to help manage *Advent of Code* puzzle projects. It automates the setup of new
puzzle days and downloads puzzle statements, making it easier to organize and work on your solutions.

## Features

- **add_day**: Set up the folder structure, template files, and download the input for a new *Advent of Code* puzzle.
- **get_statement**: Download the puzzle statement (both parts if available) for a specific day and year.

## Usage

Run the tool from the root of your workspace:

```
$ cargo run --release -p aoc-assistant -- <SUBCOMMAND> [OPTIONS]
```

### Subcommands

#### add_day

Set up the project for a new *Advent of Code* puzzle. This creates the folder structure and Rust module, and downloads
the
input data for the specified year and day. Requires the session key.

**Required arguments:**

- `--year`, `-y` — The year of the puzzle (e.g., 2023)
- `--day`, `-d` — The day of the puzzle (1-25)
- `-s`/`--session` or `-S`/`--session-file` — The session key string (see below)

**Optional arguments:**

- `--force`, `-f` — Overwrite existing files if they already exist

#### get_statement

Download the puzzle statement/instructions for the specified year and day. Requires the session key.

**Required arguments:**

- `--year`, `-y` — The year of the puzzle (e.g., 2023)
- `--day`, `-d` — The day of the puzzle (1-25)
- `-s`/`--session` or `-S`/`--session-file` — The session key string (see below)

## Session Key

To download puzzle input or statements, you need to provide your *Advent of Code* session key. You can find this key in
your browser cookies after logging in to the *Advent of Code* website.

You can provide the session key in two ways:

1. **Directly as a command-line argument:**
    - Use the `--session` or `-s` flag followed by your session key string.
2. **From a file:**
    - Store your session key in a file (e.g., `session_key.txt` in the root of your workspace) and use the
      `--session-file` or `-S` flag with the file path.

## Examples

### Example 1: Add puzzle for 2023 day 2 using session key directly

```
cargo run --release -p aoc-assistant -- add_day --year 2023 --day 2 --session 3616c7465645f5f057c1c8359d0812fa391f5cbf7724660967ee4860fa218359d872af5362e2e592e250d14a6665bf6e2d75aa666dbe67b14c1634
```

### Example 2: Add puzzle for 2023 day 2 using session key from a file

First, create a file named `session_key.txt` in the root of your workspace and paste your session key into it (no extra
spaces or newlines).

Then run:

```
cargo run --release -p aoc-assistant -- add_day --year 2023 --day 2 --session-file session_key.txt
```

### Example 3: Download the statement for 2023 day 2

```
cargo run --release -p aoc-assistant -- get_statement --year 2023 --day 2 --session 3616c7465645f5f057c1c8359d0812fa391f5cbf7724660967ee4860fa218359d872af5362e2e592e250d14a6665bf6e2d75aa666dbe67b14c1634
```

## Notes

- The tool requires network access to download puzzle input and statements.
- Your session key is sensitive—keep it private and do not share it.
- Use the `--force` flag to overwrite existing files if you want to re-initialize a puzzle folder. Be aware that this
  will cause you to lose any existing code or data in that folder.

