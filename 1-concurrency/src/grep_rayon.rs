use regex::bytes::Regex;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;
use std::fs::DirEntry;
use std::process;

// The the struct you need to use to print your results.
pub use crate::grep_result::GrepResult;
use crate::util::{recv_print_result, send_match_in_file};

pub fn grep_rayon(paths: Vec<PathBuf>, regex: Regex) {
    let (tx, rx) = sync_channel::<GrepResult>(10);

    // Spawn a thread to print the results out.
    let handle_print_result = thread::spawn(move || {
        recv_print_result(rx);
    });

    //use a rayon parallel iterator to traverse the directories
    paths.into_par_iter().for_each(|path| {
        traverse_paths_rayon(path, regex.clone(), tx.clone());
    });

    drop(tx);

    handle_print_result.join().unwrap_or_else(|_| {
        eprintln!("Error when joining threads");
        process::exit(1);
    });
}

fn traverse_paths_rayon(path: PathBuf, regex: Regex, tx: SyncSender<GrepResult>) {
    //use a rayon parallel iterator to perform a traverse on each sub-directory
    if path.is_dir() {
        if let Ok(entry) = path.read_dir() {
            let entries: Vec<DirEntry> = entry.flatten().collect();
            entries.into_par_iter().for_each(|entry| {
                traverse_paths_rayon(entry.path(), regex.clone(), tx.clone());
            })
        }
    } else {
        send_match_in_file(path, regex, tx);
    }
}