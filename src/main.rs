use clap::Parser;
use regex::bytes::Regex;
use std::path::PathBuf;

mod grep;
use grep::grep_rayon;
mod result;

#[derive(Parser, Debug)]
struct Args {
    // mode to use
    mode: String,

    // The regex pattern provided
    regex: String,

    // The paths to be searched
    paths: Vec<String>,
}

fn print_help() -> () {
    println!("RGrep Implementation \n------------------------");
    println!("Arguments");
    println!("-r : search directory for regex pattern")
}

fn main() {
    //Parse arguments, using the clap crate
    let args: Args = Args::parse();
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
        "h" => {
            print_help();
        }
        _ => {
            print_help();
        }
    }
}
