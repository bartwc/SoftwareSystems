use clap::Parser;
use rayon::prelude::*;
use regex::bytes::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::Read;
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
            if path.is_dir() {
                if let Err(e) = process_directory(&path, &regex) {
                    eprintln!("Error processing {}: {}", path.display(), e);
                }
            } else {
                if let Err(e) = process_file(&path, &regex) {
                    eprintln!("Error processing {}: {}", path.display(), e);
                }
            }
        });
    }
}

fn process_directory(dir_path: &Path, regex: &Regex) -> io::Result<()> {
    let entries: Vec<_> = std::fs::read_dir(dir_path)?.collect();

    entries.par_iter().for_each(|entry| {
        let entry = entry.as_ref().unwrap();
        let path = entry.path();

        if path.is_dir() {
            if let Err(e) = process_directory(&path, regex) {
                eprintln!("Error processing {}: {}", path.display(), e);
            }
        } else {
            if let Err(e) = process_file(&path, regex) {
                eprintln!("Error processing {}: {}", path.display(), e);
            }
        }
    });

    Ok(())
}

fn process_file(file_path: &Path, regex: &Regex) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut content = Vec::new();
    file.take(1024 * 1024 * 10) // Read up to 10 megabytes of content
        .read_to_end(&mut content)?;

    let content_str = String::from_utf8_lossy(&content);

    for (search_ctr, line) in content_str.lines().enumerate() {
        if regex.is_match(line.as_ref()) {
            let matches: Vec<_> = regex.find_iter(line.as_bytes()).collect();
            if let Some(matched) = matches.get(0) {
                let matched_str = &line[matched.start()..matched.end()];

                let result = GrepResult {
                    path: file_path.to_path_buf(),
                    search_ctr,
                    line: String::from(line),
                    matched: String::from(matched_str),
                    ranges: vec![matched.range()],
                    content: content.clone(),
                };

                println!("{}", result);
            }
        }
    }

    Ok(())
}
