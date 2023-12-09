use std::env::args;
use std::io::{stdout, Read, Write};
use std::io::{self};

use tudelft_arm_qemu_runner::Runner;

use crate::Function::{UP, LEFT, DOWN, RIGHT, TRANSMIT, RECEIVE};

/// ASCII Table - 256 characters represented by 8 Bits (1 Byte)
/// Require Sufficient Bytes to Execute Forward, Backward, Left, Right Movement
/// Using w, a, s, d keystrokes

/// "Runner" represents the PC. The Runner starts the Emulator and Enable
/// Universal Asynchronous Receiver-Transmitter (UART) communication.
fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let binary = args().nth(1).unwrap();
    let mut runner: Runner = Runner::new(&binary, false)?;

    // receive up to 4 bytes
    let mut buf = [0u8; 4];

    // loop {
    //     let num_received = runner.stream.read(&mut buf)?;
    //     // get the portion we actually received
    //     let received = &buf[0..num_received];
    //     // print it (without newline)
    //     print!("{}", String::from_utf8_lossy(received));
    //     // flush to show what's printed without having to print a newline
    //     stdout().lock().flush().unwrap();
    //
    //     // send back the bytes to the Stellaris board
    //     runner.stream.write_all(received)?;
    // }
    let mut run = false;
    let mut error = false;
    loop {
        if run == false {
            println!("
            ----------------------------------------------------------------\n
            |   Hi User! Key In Request to Initialize! Enter -help for Help |\n
            ----------------------------------------------------------------\n
            ");
            run == true;
        }

        if error == true {
            println!("Hi User! No valid request received! Enter -help for help!");
        }
        error = false;

        // Step 1 - Obtain Request from User
        let mut user_request = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut user_request).expect("Invalid Request");

        // Step 2 - Parse Command From User
        let tasks = user_request.as_str().trim().split(" ").collect::<std::vec::Vec<&str>>();
        if tasks.len() != 2 && !(user_request.as_str().trim() == "-help" || user_request.as_str().trim() == "-quit") {
            println!("Request format is invalid! Enter -help for help!");
            continue;
        }

        let mut buf = [0; 24];
        let mut output = Ok(());

        match user_request.as_str().trim() {
            "-help" => {
                println!("
                    Category    [command]       [message] or [ID]\n
                    Commands:   -help:          Display help\n
                                -w:             Move - Up\n
                                -a:             Move - Left\n
                                -s:             Move - Down\n
                                -d:             Move - Right\n
                                -t:             Transmit a Serial Message\n
                                -r:             Receive a Serial Message\n
                                -quit:          Quit Program\n

                    Message:    Your Intended Message\n

                    ID:         ID of Intended Message\n
                     ");
                continue;
            }
            "-quit" => {
                //break;
            }
            _ => {
                match tasks[0] {
                    "-w" => {
                        output = UartProtocol::transmit(&mut buf, UP, String::from(tasks[1]), 0);
                    },
                    "-a" => {
                        output = UartProtocol::transmit(&mut buf, LEFT, String::from(tasks[1]), 0);
                    },
                    "-s" => {
                        output = UartProtocol::transmit(&mut buf, DOWN, String::from(tasks[1]), 0);
                    },
                    "-d" => {
                        output = UartProtocol::transmit(&mut buf, RIGHT, String::from(tasks[1]), 0);
                    },

                    "-t" => {
                        output = UartProtocol::transmit(&mut buf, TRANSMIT, String::from(tasks[1]), 0);
                    },
                    "-r" => {
                        match tasks[1].trim().parse::<u8>() {
                            Ok(id) => {
                                output = UartProtocol::transmit(&mut buf, RECEIVE, String::from(""), id);
                            },
                            Err(_) => {
                                error = true;
                                continue;
                            },
                        }
                    },
                    _ => {
                        println!("Invalid Tasks. Enter -help for help!");
                        continue;
                    },
                }
            }

        }
    }
    // println!("
    //             ---------------------------------------------------------------------\n
    //             |             Bye User!  *** Program Has Quit ***                   |\n
    //             ---------------------------------------------------------------------\n
    //             ");
}
