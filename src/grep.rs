use crate::result::{ResultDirectory, ResultFile};
use rayon::prelude::*;
use regex::bytes::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

// recursive search function for directories
fn search_directory(root_path: &Path, regex: &Regex, count: &Arc<Mutex<usize>>) -> () {
    //make sure it is a valid entry point
    if let Ok(entries) = fs::read_dir(root_path) {
        //iterator to make it parallel
        //par_iter() for iterations over vectors and such to make parallel
        //par_bridge() turns iterators into parallel
        entries.flatten().par_bridge().for_each(|entry| {
            //same logic as in the normal function
            let path = entry.path();

            //directory so we do recursive search
            if path.is_dir() {
                let count_clone = Arc::clone(&count);
                let regex_clone = regex.clone();
                search_directory(&path, &regex_clone, &count_clone);
            //single file so we can check the content
            } else {
                if let Ok(content) = fs::read(&path) {
                    //check regex pattern in the source directory
                    if let Some(mat) = regex.find(&content) {
                        //get copy of count of instance and feed it into struct for printing
                        let result_count = count.lock().unwrap().clone();
                        let result_to_print = ResultDirectory {
                            start: mat.start(),
                            end: mat.end(),
                            content: &content,
                            path: &path,
                            count: result_count,
                        };
                        //increment the counter
                        *count.lock().unwrap() += 1;
                        //print result
                        println!("{}", result_to_print);
                    }
                }
            }
        });
    }
}

// overarching function for directory search in case there are more multiple directories
pub fn grep_rayon_directory(pathvec: Vec<PathBuf>, regex: &Regex) -> () {
    let count = Arc::new(Mutex::new(0));

    pathvec.par_iter().for_each(|path| {
        search_directory(&path, &regex, &count);
    });
}

// Helper function to read lines from a file
fn read_lines(file_path: &Path) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

// search a single file
pub fn grep_search_file(root_path: &Path, regex: &Regex, count: &Arc<Mutex<usize>>) -> () {
    let path = root_path.to_path_buf();
    if let Ok(lines) = read_lines(root_path) {
        lines.par_bridge().for_each(|line| {
            let line_str = &line.unwrap();
            if let Some(_) = regex.find(line_str.as_bytes()) {
                let result_count = count.lock().unwrap().clone();
                let result_to_print = ResultFile {
                    path: &path,
                    count: result_count,
                    line: line_str
                };
                *count.lock().unwrap() += 1;
                println!("{}", result_to_print);
            }
        })
    }
}

pub fn grep_rayon_file(pathvec: Vec<PathBuf>, regex: &Regex) -> () {
    let count = Arc::new(Mutex::new(0));

    pathvec.par_iter().for_each(|path| {
        if !path.is_dir() {
            grep_search_file(path, &regex, &count)
        } else {
            println!("{:?} is a directory. Please use recursive search -r", path);
        }
    });
}
