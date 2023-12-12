use std::env::args;
use std::io;
use std::io::{stdout, Read, Write};
use std::thread::sleep;
use std::time::Duration;

use tudelft_arm_qemu_runner::Runner;
use common_lib::{DataFrame, deserialise, PayLoad, serialise};
use common_lib::Direction::{Down, Left, Right, Up};

fn main()  {
    tracing_subscriber::fmt::init();
    color_eyre::install().unwrap();

    let binary = args().nth(1).unwrap();
    let mut runner: Runner = Runner::new(&binary, false).unwrap();
    let mut recv_data = Vec::<u8>::new();
    // receive up to 256 bytes
    let mut buf = [0u8; 256];

    // sleep(Duration::from_millis(10));
    // let a :u32 = 456765456;
    // let serialised = serialise(a);
    // runner.stream.write_all(serialised.as_slice())?;
    //
    // runner.stream.write_all(serialised.as_slice())?;
    //
    // runner.stream.write_all(serialised.as_slice())?;



    let mut is_first = true;
    loop {
        if is_first {
            println!("
            ------------------------------------------------------\n
            |  Hi User! Key In Request or enter -help for Help!  |\n
            ------------------------------------------------------\n
            ");
            //stdout().lock().flush().unwrap();
            is_first = false;
        }

        // Step 1 - Obtain Request from User
        let mut user_request = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut user_request).expect("Invalid Request");

        // Step 2 - Parse Command From User
        let tasks = user_request.as_str().trim().split(" ").collect::<std::vec::Vec<&str>>();



        match user_request.as_str().trim() {
            "-help" | "help" | "-h" | "h" => {
                println!("
                    Category    [command]       [message] or [ID]\n
                    Commands:   -help:          Display help\n
                                -w:             Move - Up\n
                                -a:             Move - Left\n
                                -s:             Move - Down\n
                                -d:             Move - Right\n
                                -t:             Transmit Message to Change Page View\n
                                -r:             Receive Total Number of Steps\n
                                -c:             Clear Step Count to Zero\n
                                -quit:          Quit Program\n
                     ");
                //stdout().lock().flush().unwrap();
            }
            "-quit" | "quit" | "-q" | "q" => {
                break
            }
            _ => {
                match tasks[0] {
                    "-w" | "w" => {
                        let msg = DataFrame{
                            payload: PayLoad::TakeStep(Up),
                        };
                        let serialised = serialise(msg);
                        runner.stream.write_all(serialised.as_slice()).unwrap();
                    },
                    "-a" | "a" => {
                        let msg = DataFrame{
                            payload: PayLoad::TakeStep(Left),
                        };
                        let serialised = serialise(msg);
                        runner.stream.write_all(serialised.as_slice()).unwrap();
                    },
                    "-s" | "s" => {
                        let msg = DataFrame{
                            payload: PayLoad::TakeStep(Down),
                        };
                        let serialised = serialise(msg);
                        runner.stream.write_all(serialised.as_slice()).unwrap();
                    },
                    "-d" | "d" => {
                        let msg = DataFrame{
                            payload: PayLoad::TakeStep(Right),
                        };
                        let serialised = serialise(msg);
                        runner.stream.write_all(serialised.as_slice()).unwrap();
                    },

                    "-t" | "t" => {
                        let msg = DataFrame{
                            payload: PayLoad::ChangeView,
                        };
                        let serialised = serialise(msg);
                        runner.stream.write_all(serialised.as_slice()).unwrap();
                    },
                    "-c" | "c" => {
                        let msg = DataFrame{
                            payload: PayLoad::Clear,
                        };
                        let serialised = serialise(msg);
                        runner.stream.write_all(serialised.as_slice()).unwrap();
                    },
                    "-r" => {

                    },
                    _ => {
                        println!("Invalid Tasks. Enter -help for help!");
                    },
                }
            }

        }



        // flush to show what's printed without having to print a newline
        //stdout().lock().flush().unwrap();

        // send back the bytes to the Stellaris board
        // runner.stream.write_all(serialise(456765456).as_slice())?;
        // let a = DataFrame{
        //     payload: PayLoad::ChangeView,
        //     sequence_nr: 456765456,
        // };
        // let serialised = serialise(a);
        // runner.stream.write_all(serialised.as_slice())?;
    }
}
