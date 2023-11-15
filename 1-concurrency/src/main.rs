use clap::Parser;
use regex::bytes::Regex;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::fs;
// use std::time::Instant;

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

static NUM_THREADS:AtomicUsize = AtomicUsize::new(0);

fn main() {
    // let now = Instant::now();
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

    // Fetch the number of cores in the system
    let available_parallelism =thread::available_parallelism().expect("failed finding the number of available logical cores").get();

    // Use a channel to transfer the results from different threads to the thread that is responsible to printing them out.
    let (tx, rx) = channel::<GrepResult>();

    // Spawn a thread to print the results out.
    let handel_print_result = thread::spawn(move || {
        let mut search_ctr : usize = 0;
        for mut received in rx {
            received.search_ctr = search_ctr;
            println!("{}", received);
            search_ctr=search_ctr+1;
        }
    });

    // Spawn a thread to traverse a new path when there are idle cores in the system
    // When there is no idle core, the traverse continue in the main thread.
    for path in paths {
        if NUM_THREADS.load(SeqCst) < available_parallelism {
            NUM_THREADS.fetch_add(1, SeqCst);
            let regex = regex.clone();
            let tx = tx.clone();
            thread::spawn(move || {
                traverse_paths(path, regex, tx, available_parallelism);
                NUM_THREADS.fetch_sub(1, SeqCst);
            });
        }
        else {
            traverse_paths(path, regex.clone(), tx.clone(), available_parallelism);
        }
    }

    // Drop the tx in the main thread to allow the print_result thread to finish
    drop(tx);
    handel_print_result.join().unwrap();

    // let elapsed_time = now.elapsed();
    // println!("Running took {} ms.", elapsed_time.as_millis());
}

fn traverse_paths(path: PathBuf, regex: Regex, tx: Sender<GrepResult>, num_cores: usize){
    // Spawn a thread to traverse a new path when there are idle cores in the system
    // When there is no idle core, the traverse continue in the old thread.
    if path.is_dir() {
        for entry in path.read_dir().expect("read_dir call failed") {
            if let Ok(entry) = entry {
                if NUM_THREADS.load(SeqCst) < num_cores {
                    NUM_THREADS.fetch_add(1, SeqCst);
                    let regex = regex.clone();
                    let tx = tx.clone();
                    thread::spawn(move || {
                        traverse_paths(entry.path(), regex, tx, num_cores);
                        NUM_THREADS.fetch_sub(1, SeqCst);
                    });
                }
                else {
                    traverse_paths(entry.path(), regex.clone(), tx.clone(), num_cores);
                }
            }
        }
    }
    else {
        let file = fs::read(path.as_path())
            .expect("reading file failed");
        // Send the result to the channel when a match is found.
        for each in regex.find_iter(file.as_slice()) {
            let mut result: GrepResult = GrepResult {
                path: path.clone(),
                content: file.clone(),
                ranges: Vec::new(),
                search_ctr: 0,
            };
            result.ranges.push(each.range());
            tx.send(result).unwrap();
        }
    }
}