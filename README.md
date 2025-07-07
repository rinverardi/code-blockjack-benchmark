# Blockjack Benchmarks

![](https://img.shields.io/badge/Status-Experimental-orange)

## Usage

Clean up:

	$ cargo clean

Run the tests:

	$ cargo test --release

### Compute Benchmarks

Run the compute benchmarks:

	$ cargo bench
	$ firefox target/criterion/blockjack/report/index.html

### Memory Benchmarks

Run the memory benchmarks:

	$ cargo build --release
	$ heaptrack target/release/blockjack
