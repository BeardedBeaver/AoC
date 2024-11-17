# ADVENT OF CODE

## Running this code:

Input files are expected to be in a folder pointed by `AOC_{year}_INPUT_PATH`
(`AOC_2023_INPUT_PATH` for 2023).

```
export AOC_2023_INPUT_PATH=/path/to/folder-with-input
cargo test
cargo run
```

Or if you're on Windows PowerShell:

```
$env:AOC_2023_INPUT_PATH="D:/path/to/folder-with-input"
```

You can run a specific year, day or part:

```
cargo run --day 5
```

this will run all the day 5 for all available years

To run an optimized build:

```
cargo run --release -- -y 2023 -d 5 -p 2
```

*Ofc we need to run day 2024-5-2 in release :)*

