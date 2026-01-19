
# Performance Test

A performance test to compare the SDK of different languages.
Each script will generate one million decisions for the flag `buy_button` from the sandbox datafile. The scripts can be adjusted to make the same number of decisions for the flags from other data files.
The user IDs will start with the phrase "user" followed by a number from 0 to 999,999.
Note that the times also include reading the datafile from disk and initializing the SDK. However, this should only be a fraction of the total time for one million decisions.

## Instructions

To run the scripts for each language, use the following commands:

### Rust
```sh
cargo build --release
/usr/bin/time -v target/release/performance-test
```

### Python
```sh
pip install -r requirements.txt
/usr/bin/time -v python src/main.py
```

### JavaScript
```sh
npm install
/usr/bin/time -v node src/main.js
```

### Go
```sh
go build -o bin/main src/main.go
/usr/bin/time -v bin/main
```

## Results

Here is a comparison of this **unofficial** Rust SDK to the official SDKs in other languages.

### Sandbox account

These are the results for my personal sandbox account containing 7 feature flags and 4 experiments.

| Language   | Duration (1M) | Average latency / decision | Decisions / second | Peak memory usage |
| :--------- | ------------: | -------------------------: | -----------------: | ----------------: |
| Rust       |        0.17 s |                    0.17 μs |               5.8M |              3 MB |
| Go         |        2.80 s |                    2.80 μs |               357k |             15 MB |
| JavaScript |        3.18 s |                    3.18 μs |               314k |            132 MB |
| Python     |       22.57 s |                   22.57 μs |                44k |             27 MB |

### Real customer account

These are the results for a real customer account. Let's call them `Hello, World!`. Their account contains 741 feature flags and 349 experiments.

| Language   | Duration (1M) | Average latency / decision | Decisions / second | Peak memory usage |
| :--------- | ------------: | -------------------------: | -----------------: | ----------------: |
| Rust       |        0.17 s |                    0.17 μs |               5.8M |             11 MB |
| Go         |       13.52 s |                   13.52 μs |                74k |             33 MB |
| JavaScript |        9.71 s |                    9.71 μs |               103k |            171 MB |
| Python     |       67.87 s |                   67.87 μs |                15k |             42 MB |
