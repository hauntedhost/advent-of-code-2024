<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2024

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2024 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2024/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2024/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2024/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2024/day/4) | ⭐ |   |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `39.8µs` | `51.8µs` |
| [Day 2](./src/bin/02.rs) | `40.5µs` | `223.0µs` |
| [Day 3](./src/bin/03.rs) | `147.4µs` | `334.6µs` |
| [Day 4](./src/bin/04.rs) | `655.1µs` | `-` |

**Total: 1.49ms**
<!--- benchmarking table --->

---

## Usage

### Common

```sh
cargo today # Scaffold, download input and read puzzle of the day!
cargo scaffold [day] --download # Scaffold and download input for specified day
cargo solve [day] # Solve for specified day
cargo solve [day] --submit [part] # Solve for day and submit/check solution
cargo test --bin [day] # Run tests for specified day
cargo time --all --store # Benchmark all solutions and update readme
```

### All Commands
```sh
cargo all # Solve for all days
cargo clippy # Lint code
cargo download [day] # Download input for day
cargo fmt # Format code
cargo read [day] # Read puzzle description for specified day
cargo scaffold [day] --download # Scaffold and download input for specified day
cargo solve [day] # Solve for specified day
cargo solve [day] --dhat # Solve and generate dhat-heap.json for https://nnethercote.github.io/dh_view/dh_view.html
cargo solve [day] --release # Solve for day with optimized release
cargo solve [day] --submit [part] # Solve for day and submit/check solution
cargo test # Run all tests
cargo test --bin [day] # Run tests for specified day
cargo time --all --store # Benchmark all solutions and update readme
cargo today # Scaffold, download input and read puzzle of the day!
```

## Useful crates

- [itertools](https://crates.io/crates/itertools): Extends iterators with extra methods and adaptors. Frequently useful for aoc puzzles.
- [regex](https://crates.io/crates/regex): Official regular expressions implementation for Rust.
- [blessred.rs](https://blessed.rs/crates): A curated list of popular crates

---
👻 [haunted.host](https://www.haunted.host)
