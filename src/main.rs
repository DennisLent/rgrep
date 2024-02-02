use argparse::{ArgumentParser, List, Store, StoreTrue};
use regex::bytes::Regex;
use std::path::PathBuf;

mod grep;
use grep::{grep_rayon_directory, grep_rayon_file};
mod result;

#[derive(Debug)]
struct Args {
    // The regex pattern provided
    regex: String,

    // The paths to be searched
    paths: Vec<String>,

    // determine if it will be a search in a directory or a file
    recursive: bool,

    // print filenames only
    filename_only: bool,

    // print lines and line numbers
    linenumbers: bool,

    // print only the count of matching lines
    linecount: bool,

    // print lines that are not matching
    inverse: bool,

    // how many lines to print after the match
    context_lines: Option<usize>,
}

impl Args {
    //function to parse arguments from the command line and store them in a struct
    fn parse() -> Args {
        // intialize an empty argument struct
        let mut args = Args {
            regex: String::new(),
            paths: Vec::new(),
            recursive: false,
            filename_only: false,
            linenumbers: false,
            linecount: false,
            inverse: false,
            context_lines: None,
        };

        let mut lines: usize = 0;

        {
            // check all the options to create the arguments struct
            let mut parser = ArgumentParser::new();
            parser.set_description("===== A simple grep tool built uing Rust =====");

            parser
                .refer(&mut args.regex)
                .add_argument("regexPattern", Store, "Regex pattern to search for")
                .required();

            parser
                .refer(&mut args.paths)
                .add_argument("path", List, "Path to file or directory")
                .required();

            parser
                .refer(&mut args.recursive)
                .add_option(&["-r"], StoreTrue, "Search a directory");

            parser.refer(&mut args.filename_only).add_option(
                &["-l"],
                StoreTrue,
                "Print filenames only",
            );

            parser.refer(&mut args.linenumbers).add_option(
                &["-n"],
                StoreTrue,
                "Print lines with match and linenumbers",
            );

            parser.refer(&mut args.linecount).add_option(
                &["-c"],
                StoreTrue,
                "Print count of lines with matching criteria",
            );

            parser.refer(&mut args.inverse).add_option(
                &["-v"],
                StoreTrue,
                "Print lines with no match match (inverse search)",
            );

            parser
                .refer(&mut lines)
                .add_option(&["-A"], Store, "Print n lines after the match");

            parser.parse_args_or_exit();
        }
        args.context_lines = if lines > 0 { Some(lines) } else { None };

        args
    }
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    if args.paths.is_empty() || args.regex.is_empty() {
        println!("Regex pattern and Path are necessary");
    }

    let regex = Regex::new(&args.regex).unwrap();

    // Get the paths that we should search
    let paths = if args.paths.is_empty() {
        //If no paths were provided, we search the current path
        vec![std::env::current_dir().unwrap()]
    } else {
        // Take all paths from the command line arguments, and map the paths to create PathBufs
        args.paths.iter().map(PathBuf::from).collect()
    };

    //match args to determine the function
    match args.recursive{
        true => grep_rayon_directory(paths, &regex, args),
        false => grep_rayon_file(paths, &regex, args),
    }
}
