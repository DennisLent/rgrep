use argparse::{ArgumentParser, List, Store, StoreTrue};
use regex::bytes::Regex;
use std::path::PathBuf;

mod grep;
use grep::grep_rayon;
mod result;

#[derive(Debug)]
struct Args {
    // mode to use
    mode: String,

    // The regex pattern provided
    regex: String,

    // The paths to be searched
    paths: Vec<String>,

    // determine if it will be a search in a directory or a file
    recursive: bool,
}

impl Args {
    //function to parse arguments from the command line and store them in a struct
    fn parse() -> Args {
        let mut args = Args {
            mode: String::new(),
            regex: String::new(),
            paths: Vec::new(),
            recursive: false,
        };

        {
            let mut parser = ArgumentParser::new();
            parser.set_description("A simple grep tool built uing Rust");

            parser
                .refer(&mut args.mode)
                .add_argument("mode", Store, "yada yada")
                .required();

            parser
                .refer(&mut args.regex)
                .add_argument("regex pattern", Store, "Regex pattern to search for")
                .required();

            parser
                .refer(&mut args.paths)
                .add_argument("path", List, "Path to file or directory")
                .required();

            parser.parse_args_or_exit();

        }
        args
    }
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    let regex = Regex::new(&args.regex).unwrap();

    // Get the paths that we should search
    let paths = if args.paths.is_empty() {
        //If no paths were provided, we search the current path
        vec![std::env::current_dir().unwrap()]
    } else {
        // Take all paths from the command line arguments, and map the paths to create PathBufs
        args.paths.iter().map(PathBuf::from).collect()
    };

    match args.mode.as_str() {
        "r" => {
            grep_rayon(paths, &regex);
        }
        _ => {
            println!("oof");
        }
    }
}
