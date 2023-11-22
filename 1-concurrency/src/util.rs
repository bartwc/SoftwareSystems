use std::sync::mpsc::{Receiver, SyncSender};
use std::path::{PathBuf};
use regex::bytes::Regex;
use std::fs;
use std::ops::Range;

pub use crate::grep_result::GrepResult;

pub fn recv_print_result(rx: Receiver<GrepResult>) {
    for (search_ctr, mut received) in rx.iter().enumerate() {
        received.search_ctr = search_ctr;
        println!("{}", received);
    }
}

pub fn send_match_in_file(path: PathBuf, regex: Regex, tx: SyncSender<GrepResult>) {
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