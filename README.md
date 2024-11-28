# Rust HTTPS retrieval and JSON parsing

Trading Specific Algorithms
Review the Binance European Options API documentation at
https://binance-docs.github.io/apidocs/voptions/en/.

Then:

◦ Retrieve data from the endpoint:
GET /eapi/v1/ticker

◦ Write a parsing algorithm for the instrument statistics and print the result

◦ Measure the parsing speed for a single entry and evaluate the algorithmic complexity
of your approach. Document any potential optimizations aimed at achieving low
latency.

## Requirements

*install Rust

## Building

**Clone the repo**

```$ git clone https://github.com/fgsfdgsdgs/test2.git```

To run the code with debug output:
```
cargo build --features debug
cargo run --features debug
```
otherwise:
```
cargo build --release
cargo run --release
```


**Tests**
```
cargo test
```


