<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/Hekkfern/advent-of-code-rust">
    <img src="docs/images/logo.png" alt="Logo" width="300" height="300">
  </a>

<h3 align="center">Advent Of Code Rust Challenge</h3>

  <p align="center">
    <i>Advent Of Code</i> solutions made in Rust language
    <br />
    <a href="https://github.com/Hekkfern/advent-of-code-rust/discussions">Discuss</a>
    ·
    <a href="https://github.com/Hekkfern/advent-of-code-rust/issues">Report Bug</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li><a href="#getting-started">Getting Started</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#faq">FAQ</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

This project has been created with the main goal to provide a easy-to-use development platform to solve [Advent of code](https://adventofcode.com/) puzzles in Rust.

Features:

* Latest Rust features
* Parsing of the input files, so you can use your own data to solve the puzzles.
* Release build configured for maximum speed.
* Use of modern CPU instructions to minimize the execution times.
* Compliant with quality tools recommendations from `cargo-clippy`
* CI/CD multi-platform pipelines to validate that everything works as it should in every platform available.
* Unit-testing: create unit tests to validate the example inputs, or to validate that your future changes don't break your code.
* Multi-platform: designed to be usable in Windows, Linux and MacOs.
* Dockerization: if you don't want to pollute your computer with all the required development tools, there is a Docker
  image available to start developing in a matter of seconds.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

Tested on:

| Platform | OS                              |
| -------- | ------------------------------- |
| Windows  | Windows 11                      |
| Linux    | Ubuntu 24.04                    |
| Apple    | macOS Sequoia 15  (M processor) |

<!-- GETTING STARTED -->

## Getting Started

Install [Git LFS](https://git-lfs.com/) tool in your machine.

Clone the repository:

```bash
git clone git@github.com:Hekkfern/advent-of-code-rust.git
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE INSTRUCTIONS -->

## Usage

### Run a puzzle solution

To run the solution for the Part 1 of a puzzle:

```bash
cargo run --release --bin aoc_<year>_<day>_p1
```

or, to run the solution for the Part 2 of a puzzle:

```bash
cargo run --release --bin aoc_<year>_<day>_p2
```

where:
* <year> is the year (format 20XX) of the puzzle.
* <day> is the day (from 1 to 25) of the puzzle.

### Run the unit tests for one of the puzzles

```bash
cargo test --release -p aoc_<year>_<day>
```

where:
* <year> is the year (format 20XX) of the puzzle.
* <day> is the day (from 1 to 25) of the puzzle.

### Add a new *Advent of Code* puzzle

Read the docs of the **aoc-assistant** internal tool in its [README.md](./tools/aoc-assistant/README.md) page.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- FAQ -->

## FAQ

Read [FAQ.md](./docs/FAQ.md) page.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->

## Roadmap

See the [open issues](https://github.com/Hekkfern/advent-of-code-rust/issues) for a list of proposed features (and known issues).

- [Top Feature Requests](https://github.com/Hekkfern/advent-of-code-rust/issues?q=label%3Aenhancement+is%3Aopen+sort%3Areactions-%2B1-desc) (Add your votes using the 👍 reaction)
- [Top Bugs](https://github.com/Hekkfern/advent-of-code-rust/issues?q=is%3Aissue+is%3Aopen+label%3Abug+sort%3Areactions-%2B1-desc) (Add your votes using the 👍 reaction)
- [Newest Bugs](https://github.com/Hekkfern/advent-of-code-rust/issues?q=is%3Aopen+is%3Aissue+label%3Abug)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Read [CONTRIBUTING.md](./docs/CONTRIBUTING.md) page.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

If you want to suggest anything or discuss some ideas about this project, feel free to open a topic
in [Discussions](https://github.com/Hekkfern/advent-of-code-rust/discussions)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->

[contributors-shield]: https://img.shields.io/github/contributors/Hekkfern/advent-of-code-rust.svg?style=for-the-badge
[contributors-url]: https://github.com/Hekkfern/advent-of-code-rust/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Hekkfern/advent-of-code-rust.svg?style=for-the-badge
[forks-url]: https://github.com/Hekkfern/advent-of-code-rust/network/members
[stars-shield]: https://img.shields.io/github/stars/Hekkfern/advent-of-code-rust.svg?style=for-the-badge
[stars-url]: https://github.com/Hekkfern/advent-of-code-rust/stargazers
[issues-shield]: https://img.shields.io/github/issues/Hekkfern/advent-of-code-rust.svg?style=for-the-badge
[issues-url]: https://github.com/Hekkfern/advent-of-code-rust/issues
[license-shield]: https://img.shields.io/github/license/Hekkfern/advent-of-code-rust.svg?style=for-the-badge
[license-url]: https://github.com/Hekkfern/advent-of-code-rust/blob/master/LICENSE.txt
