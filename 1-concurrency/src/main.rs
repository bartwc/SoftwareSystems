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
    let content: Vec<u8> = io::BufReader::new(file).bytes().collect::<Result<_, _>>()?;

    // Ensure content length and thread count are both greater than zero
    let chunk_size = content.len() / rayon::current_num_threads().max(1);
    if chunk_size > 0 {
        content.par_chunks_exact(chunk_size).for_each(|chunk| {
            for (search_ctr, line) in chunk.split(|&b| b == b'\n').enumerate() {

                let line = String::from_utf8_lossy(line);

                if regex.is_match(line.as_bytes()) {
                    let matched = regex.find(line.as_bytes()).unwrap();
                    let result = GrepResult {
                        path: file_path.to_path_buf(),
                        search_ctr,
                        line: line.to_string(),
                        matched: String::from_utf8_lossy(&line.as_bytes()[matched.start()..matched.end()])
                            .to_string(),
                        ranges: vec![matched.range()],
                        content: chunk.to_vec(),  // Pass a reference to the relevant part of the content
                    };

                    println!("{}", result);
                }
            }
        });
    }
    Ok(())
}
