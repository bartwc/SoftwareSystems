use clap::Parser;
use regex::bytes::Regex;
use std::path::PathBuf;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::fs;
use std::ops::Range;
use std::process;
// use std::time::Instant;

// The the struct you need to use to print your results.
pub use crate::grep_result::GrepResult;