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

	$ docker run -itv ./results:/blockjack/target/criterion blockjack.benchmark-compute
	$ firefox results/report/index.html

### Memory Benchmarks

Run the memory benchmarks:

	$ cargo build --release

	$ heaptrack target/release/blockjack naive
	$ heaptrack target/release/blockjack secure

For reproducible results, the dockerized version should be used:

	$ docker build -f Dockerfile.benchmark-memory -t blockjack.benchmark-memory .

	$ docker run -itv ./results/naive:/blockjack/results blockjack.benchmark-memory naive
	$ heaptrack_gui results/naive/heaptrack.gz

	$ docker run -itv ./results/secure:/blockjack/results blockjack.benchmark-memory secure
	$ heaptrack_gui results/secure/heaptrack.gz
