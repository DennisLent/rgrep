use crate::result::Result;
use rayon::prelude::*;
use regex::bytes::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

fn search_regex(root_path: &Path, regex: &Regex, count: &Arc<Mutex<usize>>) -> () {
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
                search_regex(&path, &regex_clone, &count_clone);
            //single file so we can check the content
            } else {
                if let Ok(content) = fs::read(&path) {
                    //check regex pattern in the source directory
                    if let Some(mat) = regex.find(&content) {
                        //get copy of count of instance and feed it into struct for printing
                        let result_count = count.lock().unwrap().clone();
                        let result_to_print = Result {
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

pub fn grep_rayon(pathvec: Vec<PathBuf>, regex: &Regex) -> () {
    //println!("grep rayon running");
    let count = Arc::new(Mutex::new(0));

    pathvec.par_iter().for_each(|path| {
        search_regex(&path, &regex, &count);
    });
}
