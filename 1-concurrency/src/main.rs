use clap::Parser;
use regex::bytes::Regex;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;
use std::fs;
use std::fs::DirEntry;
use std::ops::Range;
use std::process;

// The the struct you need to use to print your results.
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
    //Parse arguments, using the clap crate
    let args: Args = Args::parse();
    let regex = Regex::new(&args.regex).unwrap_or_else(|_| {
        eprintln!("Invalid regular expression, please try again.");
        process::exit(1)
    });

    // Get the paths that we should search
    let paths = if args.paths.is_empty() {
        //If no paths were provided, we search the current path
        vec![std::env::current_dir().unwrap_or_else(|_| {
            eprintln!("Unable to find current path.");
            process::exit(1);
        })]
    } else {
        // Take all paths from the command line arguments, and map the paths to create PathBufs
        args.paths.iter().map(PathBuf::from).collect()
    };

    // Use a channel to transfer the results from different threads to the thread that is responsible to printing them out.
    // A bounded channel is used to prevent buffering all the results until the end
    let (tx, rx) = sync_channel::<GrepResult>(10);

    // Spawn a thread to print the results out.
    let handle_print_result = thread::spawn(move || {
        for (search_ctr, mut received) in rx.iter().enumerate() {
            received.search_ctr = search_ctr;
            println!("{}", received);
        }
    });

    //use a rayon parallel iterator to traverse the directories
    paths.into_par_iter().for_each(|path| {
        traverse_paths(path, regex.clone(), tx.clone());
    });

    drop(tx);

    handle_print_result.join().unwrap_or_else(|_| {
        eprintln!("Error when joining threads");
        process::exit(1);
    });
}

fn traverse_paths(path: PathBuf, regex: Regex, tx: SyncSender<GrepResult>) {
    //use a rayon parallel iterator to perform a traverse on each sub-directory
    if path.is_dir() {
        if let Ok(entry) = path.read_dir() {
            let entries: Vec<DirEntry> = entry.flatten().collect();
            entries.into_par_iter().for_each(|entry| {
                traverse_paths(entry.path(), regex.clone(), tx.clone());
            })
        }
    } else {
        let file = fs::read(path.as_path())
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                vec![]
            });

        let mut vec_match: Vec<Range<usize>> = Vec::new();
        for each in regex.find_iter(file.as_slice()) {
            vec_match.push(each.range());
        }
        // Send the result to the channel when a match is found.
        if !vec_match.is_empty() {
            let result: GrepResult = GrepResult {
                path: path.clone(),
                content: file.clone(),
                ranges: vec_match,
                search_ctr: 0,
            };
            tx.send(result).unwrap_or_else(|_| { eprintln!("Error when sending a result to channel"); });
        }
    }
}