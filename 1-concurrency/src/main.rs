use clap::Parser;
use regex::bytes::Regex;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::fs;
use std::ops::Range;
use std::process;
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
    let regex = Regex::new(&args.regex).unwrap_or_else(|_|{
        eprintln!("Invalid regular expression, please try again.");
        process::exit(1)
    });

    // Get the paths that we should search
    let paths = if args.paths.is_empty() {
        //If no paths were provided, we search the current path
        vec![std::env::current_dir().unwrap_or_else(|_|{
            eprintln!("Unable to find current path.");
            process::exit(1);
        })]
    } else {
        // Take all paths from the command line arguments, and map the paths to create PathBufs
        args.paths.iter().map(PathBuf::from).collect()
    };

    // Fetch the number of cores in the system
    let available_parallelism = if let Ok(num_cores) = thread::available_parallelism(){
        num_cores.get()
    } else {
        eprintln!("Unable to find available parallelism. The program will continue as single-threaded.");
        1
    };

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
    handel_print_result.join().unwrap_or_else(|_|{
        eprintln!("Error when joining threads");
        process::exit(1);
    });

    // let elapsed_time = now.elapsed();
    // println!("Running took {} ms.", elapsed_time.as_millis());
}

fn traverse_paths(path: PathBuf, regex: Regex, tx: Sender<GrepResult>, num_cores: usize){
    // Spawn a thread to traverse a new path when there are idle cores in the system
    // When there is no idle core, the traverse continue in the old thread.
    if path.is_dir() {
        if let Ok(entry) = path.read_dir(){
            for entry in entry {
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
    }
    else {
        let file = fs::read(path.as_path())
            .unwrap_or_else(|err|{
                eprintln!("{}", err);
                vec![]
            });

        let mut vec_match: Vec<Range<usize>> = Vec::new();
        for each in regex.find_iter(file.as_slice()) {
            vec_match.push(each.range());
        }
        // Send the result to the channel when a match is found.
        if !vec_match.is_empty(){
            let result: GrepResult = GrepResult {
                path: path.clone(),
                content: file.clone(),
                ranges: vec_match,
                search_ctr: 0,
            };
            tx.send(result).unwrap_or_else(|_|{eprintln!("Error when sending a result to channel");});
        }
    }
}