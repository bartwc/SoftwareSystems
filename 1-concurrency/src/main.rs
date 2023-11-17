use clap::Parser;
use rayon::prelude::*;
use regex::bytes::Regex;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

// The struct you need to use to print your results.
pub use crate::grep_result::GrepResult;

mod grep_result;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The regex pattern that the user provided
    regex: String,

    /// The paths in which mygrep should search, if empty, in the current directory
    paths: Vec<String>,
}

fn main() {
    // Parse arguments, using the clap crate
    let args: Args = Args::parse();
    let regex = Regex::new(&args.regex).unwrap();

    // Get the paths that we should search
    let paths = if args.paths.is_empty() {
        vec![PathBuf::from(std::env::current_dir().unwrap())]
    } else {
        args.paths.iter().map(PathBuf::from).collect()
    };

    // Use Rayon to parallelize the processing of paths
    if !paths.is_empty() && rayon::current_num_threads() > 0 {
        paths.par_iter().for_each(|path| {
            process_path(path, &regex);
        });
    }
}

fn process_path(path: &Path, regex: &Regex) {
    if path.is_dir() {
        process_directory(path, regex);
    } else {
        process_file(path, regex);
    }
}

fn process_directory(dir_path: &Path, regex: &Regex) {
    let entries: Vec<_> = fs::read_dir(dir_path)
        .unwrap_or_else(|e| panic!("Error reading directory {}: {}", dir_path.display(), e))
        .collect();

    entries.par_iter().for_each(|entry| {
        let entry = entry.as_ref().unwrap();
        let path = entry.path();

        if path.is_dir() {
            process_directory(&path, regex);
        } else {
            process_file(&path, regex);
        }
    });
}

fn process_file(file_path: &Path, regex: &Regex) {
    let file = File::open(file_path)
        .unwrap_or_else(|e| panic!("Error opening file {}: {}", file_path.display(), e));

    let mut content = Vec::new();
    let _ = file.take(1024 * 1024 * 10).read_to_end(&mut content); // Read up to 10 megabytes of content

    if regex.is_match(&content) {
        let matches: Vec<_> = regex.find_iter(&content).collect();

        let result = GrepResult {
            path: file_path.to_path_buf(),
            content: content.clone(),
            ranges: matches.iter().map(|m| m.start()..m.end()).collect(),
            search_ctr: matches.len(),
            line: "".to_string(), // You can extract the line if needed
            matched: "".to_string(), // You can extract the matched substring if needed
        };

        println!("{}", result);
    }
}
