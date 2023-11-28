use clap::Parser;
use regex::bytes::Regex;
use std::path::PathBuf;
use std::process;

use crate::grep_rayon::grep_rayon;
use crate::grep_std::grep_std_only;
use crate::GrepVariant::{GrepRayon, GrepStd};

mod grep_result;
mod grep_std;
mod grep_rayon;
mod util;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The regex pattern that the user provided
    regex: String,

    /// The paths in which mygrep should search, if empty, in the current directory
    paths: Vec<String>,
}

#[derive(PartialEq)]
enum GrepVariant {
    GrepStd,
    GrepRayon,
}

const GREP_CHOICE: GrepVariant = GrepRayon; //choose between the base implementation and the rayon implementation

fn main() {
    // let now = Instant::now();
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

    //Invoke different implementations based on the selected value of GREP_CHOICE
    if GREP_CHOICE == GrepRayon {
        grep_rayon(paths, regex);
    } else if GREP_CHOICE == GrepStd {
        grep_std_only(paths, regex);
    }
}

// cargo run -- [mM]icrodevice ./examples/example1/