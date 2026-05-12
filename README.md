# Zirc — Zero-Copy CSV Scanner

A high-performance, byte-level CSV scanner written in Rust from first principles.  
No dependencies on the `csv` crate. Built to approach raw disk read speed.

## Benchmark

| File Size | Rows | Speed | Time |
|-----------|------|-------|------|
| 224 MB | 2,000,000 | 287 MB/s | 779ms |

> Tested on a standard laptop. The bottleneck is disk I/O, not the parser.

## How It Works

Most CSV parsers allocate strings for every field on every row.  
`zirc` does not. It stores byte offsets (`start`, `end`) into the raw buffer — zero heap allocation per field.

```
Buffer: [74,111,104,110,44,50,53,10]
Field:  { s: 0, e: 4 }  →  "John"  (no copy, just a slice)
```

This is why it is fast.

## Installation

clone and build:

```bash
git clone https://github.com/ahmadzafarcs/zirc
cd zirc
cargo build --release
```

## Usage

```bash
cargo run --file data.csv --column 2 --query "Mark"
```

### Arguments

| Flag | Short | Description |
|------|-------|-------------|
| `--file` | `-f` | Path to CSV file |
| `--column` | `-c` | Column index to search (0-based, default: 2) |
| `--query` | `-q` | Value to search for |

### Example Output

```
[Match Found] ID: 479)23 | Value: Mark
-----------------------------------------
Processed 224.23 MB at 287.83 MB/s
Finished in - 779.039127ms
```

## Features

- Byte-level parsing — no string allocation per field
- Buffer boundary safe — rows split across buffer chunks are handled correctly
- Quote handling — escaped quotes `""` and quoted fields with commas supported
- Real throughput metrics — MB/s and elapsed time on every run
- Clean CLI — built with `clap`

## Limitations

- Search is exact match only — no regex or partial match yet
- Delimiter is hardcoded to `,` — configurable delimiter coming
- Output is stdout only — file output not yet supported

## Roadmap

- [ ] Iterator API — `for row in CsvParser::new("file.csv")`
- [ ] Configurable delimiter
- [ ] Regex search
- [ ] Multiple column search
- [ ] Write results to file
- [ ] Parallel processing with rayon

## What I Learned Building This

CSV parser from scratch without AI writing the code.

Along the way I learned:
- How the OS kernel feeds data from disk to program memory
- Why buffer boundary handling matters and how to fix it
- The difference between zero-copy slices and string allocation
- Why terminal output is slower than parsing itself
- How to measure real throughput and what the numbers mean

## Contributing

Found a bug or edge case? Open an issue.  
Want to add a feature from the roadmap? PRs welcome.

## License

MIT
