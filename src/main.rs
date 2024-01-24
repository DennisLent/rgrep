use clap::Parser;
use rayon::prelude::*;
use regex::bytes::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    // mode to use
    mode: String,

    // The regex pattern provided
    regex: String,

    // The paths to be searched
    paths: Vec<String>,
}

fn search_regex(root_path: &Path, regex: &Regex, count: &Arc<Mutex<usize>>) -> () {
    //println!("running rayon");
    if let Ok(entries) = fs::read_dir(root_path) {
        //iterator to make it parallel
        //par_iter() for iterations over vectors and such to make parallel
        //par_bridge() turns iterators into parallel
        entries.flatten().par_bridge().for_each(|entry| {
            //same logic as in the normal function
            let path = entry.path();
            if path.is_dir() {
                let count_clone = Arc::clone(&count);
                let regex_clone = regex.clone();
                search_regex(&path, &regex_clone, &count_clone);
            } else {
                if let Ok(content) = fs::read(&path) {
                    //check regex pattern in the source directory
                    if let Some(mat) = regex.find(&content){
                        let result_count = count.lock().unwrap().clone();
                        *count.lock().unwrap() += 1;
                        println!("[{}]: @ {:?}: {:?}", result_count, root_path, mat);
                    }
                }
            }
        });
    }
}

fn grep_rayon(pathvec: Vec<PathBuf>, regex: &Regex) -> (){
    //println!("grep rayon running");
    let count = Arc::new(Mutex::new(0));

    pathvec.par_iter().for_each(|path| {
        search_regex(&path, &regex, &count);
    });
}

fn print_help() -> (){
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
