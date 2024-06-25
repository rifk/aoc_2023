# Advent of Code 2023

My solutions for advent of code 2023 in Rust (https://adventofcode.com/2023).
This repo also includes a WASM web app, written using Yew, which uses the same solutions so you can run them in the browser. This is available at this [repo's github page](https://rifk.github.io/aoc_2023/).

## Running on command line

### Input from web

The solver can get the puzzle input from the web, this requires a https://adventofcode.com session cookie set as the `AOC_SESSION` env var. Take the session cookie from browser (using network developer tool and look at request header `Cookie: session=<session_cookie>`).
After setting the `AOC_SESSION` env var run:
```
cargo run -p day<n> -- [-o] [-t]
```
Where `n` is aoc day. The `-o` option will only run part one. The `-t` option will only run part two. Providing both options or none will run both parts.

### Input from file

The solver can also take the input from a file using the `-i` option, providing this option will ignore the `AOC_SESSION` env var.
```
cargo run -p day<n> -- [-o] [-t] -i <input_file>
```

## Project structure

- `day<n>` - solution for day `n`
- `utils` - libs for cli and getting input from web/file
- `utils-derive`- proc macro to avoid repeated boiler plate code every `day<n>` package
- `wasm-runner` - yew app to run the solver in WASM page

