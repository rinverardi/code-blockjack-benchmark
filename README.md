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

For reproducible results, the dockerized version should be used:

	$ docker build -f Dockerfile.benchmark-compute -t blockjack.benchmark-compute .
	$ docker run -it blockjack.benchmark-compute

### Memory Benchmarks

Run the memory benchmarks:

	$ cargo build --release
	$ heaptrack target/release/blockjack

For reproducible results, the dockerized version should be used:

	$ docker build -f Dockerfile.benchmark-memory -t blockjack.benchmark-memory .
	$ docker run -it blockjack.benchmark-memory
