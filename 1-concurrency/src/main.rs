use clap::Parser; // default
use regex::bytes::Regex; // default
use std::fs::File;
use std::io::{self, BufRead};
use std::io::Read;
use std::path::{Path, PathBuf}; // default

// The struct you need to use to print your results. // default
pub use crate::grep_result::GrepResult; // default

mod grep_result; // default

#[derive(Parser, Debug)] // default
#[command(version, about)] // default
// default struct
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
        // If no paths were provided, we search the current path
        vec![PathBuf::from(std::env::current_dir().unwrap())]
    } else {
        // Take all paths from the command line arguments, and map the paths to create PathBufs
        args.paths.iter().map(PathBuf::from).collect()
    };

    // Above in fn main() is default. Below is to do!()
    // Iterate through each path
    for path in paths {
        // Recursively walk the directory and find all files
        if path.is_dir() {
            if let Err(e) = process_directory(&path, &regex) {
                eprintln!("Error processing {}: {}", path.display(), e);
            }
        } else {
            if let Err(e) = process_file(&path, &regex) {
                eprintln!("Error processing {}: {}", path.display(), e);
            }
        }
    }
}

fn process_directory(dir_path: &Path, regex: &Regex) -> io::Result<()> {
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            process_directory(&path, regex)?;
        } else {
            process_file(&path, regex)?;
        }
    }
    Ok(())
}

fn process_file(file_path: &Path, regex: &Regex) -> io::Result<()> {
    let file = File::open(file_path)?;
    let content: Vec<u8> = io::BufReader::new(file).bytes().collect::<Result<_, _>>()?;

    for (search_ctr, line) in content.lines().enumerate() {
        let line = String::from_utf8_lossy(line?.as_bytes()).to_string();

        if regex.is_match(line.as_bytes()) {
            let matched = regex.find(line.as_bytes()).unwrap();
            let result = GrepResult {
                path: file_path.to_path_buf(),
                search_ctr,
                line: line.to_string(),
                matched: String::from_utf8(line.as_bytes()[matched.start()..matched.end()].to_vec())
                    .unwrap()
                    .to_string(),
                ranges: vec![matched.range()],
                content: content.clone(),
            };

            println!("{}", result);
        }
    }

    Ok(())
}