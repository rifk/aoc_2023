# WASM Runner

Simple yew application to run the advent of code solver with WASM.

### Setup

Before builder or running the app run:
```
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Running locally

To run the web app run
```
trunk serve --public-url aoc_2023
```
then the web app will be available at `http://127.0.0.1:8080/aoc_2023`.

### Running parts concurrently

When running the solver for each day part 1 and part 2 will be run concurrently. This is done use Yew's agents which use web-workers. The current implementation has some limits:
- Panics that occur in the agent task are not handled. If a panic occurs then the web app will say its calculating the part indefinitely.
- There are just two oneshot agents  which are used for all the days. This means if there is a long running part solver then changing day will not cancel it and the next days solver will only run after the long running one is complete.
- The path of the agent is hard coded to work with the github page. That is why the `--public-url aoc_2023` option is required when running locally and will not work with a different path.

