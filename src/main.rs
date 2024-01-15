use clap::Parser;
use rayon::prelude::*;
use regex::bytes::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The regex pattern that the user provided
    regex: String,

    /// The paths in which mygrep should search, if empty, in the current directory
    paths: Vec<String>,

    /// type of iterator to use
    var: String,
}

fn find_files(
    root_path: &Path,
    regex: &Regex,
    count: &Arc<Mutex<usize>>,
    thread_count: usize,
) -> () {
    //check entries in the specified path
    //println!("Finding files in {:?}", root_path);
    let mut handles = vec![];
    //keep track of how many threads have been made so far
    let mut threads_created: usize = 0;
    //println!("Threads used: {} out of {}", threads_created, thread_count);

    if let Ok(entries) = fs::read_dir(root_path) {
        for entry in entries.flatten() {
            //println!("Going through entry {:?}", entry);
            let path = entry.path();
            //edge case: is it a different directory -> spawn new thread to take care?
            if path.is_dir() {
                //we have not made more threads than allowed so we can spawn another thread
                if threads_created < thread_count {
                    //println!("Still threads to spare -> spawn a new one");
                    let count_clone = Arc::clone(&count);
                    let regex_clone = regex.clone();
                    let handle = thread::spawn(move || {
                        find_files(&path, &regex_clone, &count_clone, thread_count)
                    });
                    handles.push(handle);
                    threads_created += 1;
                } else {
                    //println!("Too many threads -> do recursion");
                    //already too many threads, therefore we only go via recursion
                    find_files(&path, &regex, &count, thread_count);
                }
            }
            //this is just a file so we see if it contains the regex
            else {
                //println!("File found");
                if let Ok(content) = fs::read(&path) {
                    if regex.is_match(&content) {
                        //copy the current count
                        let result_count = count.lock().unwrap().clone();
                        //create new GrepResult
                        let result = content;
                        //increment the new result
                        *count.lock().unwrap() += 1;
                        println!("[{}]: {:?}", result_count, result);
                    }
                }
            }
        }
        for handle in handles {
            handle.join().expect("Thread panicked");
        }
    }
}

//main grep function that implements find_files
fn grep(pathvec: Vec<PathBuf>, regex: &Regex) -> () {
    //Arc mutex to be shared
    let count = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    //max cpus, take maximum available, so either 1 or whatever max is available
    //with 1, basically not concurrent but just goes recursively
    let max_threads = std::cmp::max(1, thread::available_parallelism().unwrap().get());
    //iterate over all paths and spawn a new thread for each
    for path in pathvec {
        //println!("Checking in: {:?}", path);
        //copy values to be passed into new thread
        let regex_clone = regex.clone();
        let count_clone = Arc::clone(&count);
        let max_threads = max_threads;
        let handle =
            thread::spawn(move || find_files(&path, &regex_clone, &count_clone, max_threads));
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

fn find_files_rayon(root_path: &Path, regex: &Regex, count: &Arc<Mutex<usize>>) -> () {
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
                find_files_rayon(&path, &regex_clone, &count_clone);
            } else {
                if let Ok(content) = fs::read(&path) {
                    if regex.is_match(&content) {
                        let result_count = count.lock().unwrap().clone();
                        let result = content;
                        *count.lock().unwrap() += 1;
                        println!("[{}]: {:?}", result_count, result);
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
        find_files_rayon(&path, &regex, &count);
    });
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

    //print for debugging
    //println!("{:?}", paths);
    //println!("{}", regex);

    /* let cpus = thread::available_parallelism().unwrap().get();
    println!("Number of cpus = {}", cpus); */
    if args.var == "r"{
        grep_rayon(paths, &regex);
    }
    else if args.var == "n" {
        grep(paths, &regex);
    }

    

    //check how many files are in directory -> spawn thread for each file
    //do recursion for other folders and files
}
