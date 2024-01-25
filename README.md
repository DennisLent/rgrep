# rgrep (CURRENTLY STILL IN PROGRESS)
Rust Implementation of Grep

## Usage
Positional arguments:
  regex pattern         Regex pattern to search for
  path                  Path to file or directory

Optional arguments:
  -h,--help             Show this help message and exit
  -r                    Search a directory
  -l                    Print filenames only
  -n                    Print lines with match and linenumbers
  -c                    Print count of lines with matching criteria
  -v                    Print lines with no match match (inverse search)
  -A                    Print n lines after the match

  **Example**: ```cargo run --release [Tt]est path/to/directory```

## Functionalities to implement
- -r recursive search for directories
- -l print filenames only
- -c print count of lines with matching criteria
- -v print lines that are not matching (inverse)
- -A print n lines after matches