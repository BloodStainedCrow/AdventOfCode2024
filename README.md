# Advent of Code
These are my solutions for [AOC 2024](https://adventofcode.com/2024), written in Rust, prioritizing ease of implementation over speed of execution (though my limit for runtime was single-digit minutes on my old laptop).

[Advent of Code](https://adventofcode.com/) is a yearly programming challenge consisting of 25 days of programming puzzles.

# How to Run
[Install](https://www.rust-lang.org/tools/install) Rust and Cargo. Move into the directory of the day you are trying to run (i.e. ```day_1```) and run ```cargo run``` (or ```cargo run --release``` for release mode).

To choose whether to run part one or two, just switch the call in ```main```.

# Setup using ```nix-shell```
If you are using the [nix package manager](https://nixos.org/) you can use a [nix-shell](https://nix.dev/manual/nix/2.22/command-ref/nix-shell) to quickly and easily setup a working environment for running the code.
Just run ```nix-shell``` in the base directory of this repository.