# rgrep (CURRENTLY STILL IN PROGRESS)
Rust Implementation of Grep

## Usage
To run rgrep please run
```cargo run --release "mode" "[Rr]egex" "path/to/directory"```

## Functionalities to implement
- -r recursive search for directories
- -l print filenames only
- -c print count of lines wuith matching criteria
- -v print lines that are not matching (inverse)
- -A print n lines after matches
