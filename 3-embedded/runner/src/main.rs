use std::env::args;
use std::io;
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Duration;

use tudelft_arm_qemu_runner::Runner;
use common_lib::{DataFrame, deserialise, PayLoad, serialise};
use common_lib::Direction::{Down, Left, Right, Up};

fn main()  {
    /*
    tracing_subscriber::fmt::init(); is a crate used to collect and display tracing
    data. The fmt module provides a way to format and print tracing information.

    color_eyre::install().unwrap(); is an extension of the eyre crate, which provides
    error reporting. The install method sets up the default hook for error reporting.
    The unwrap method is called on the result to assert that this setup does not
    fail. If it does fail, the program will panic.
     */
    tracing_subscriber::fmt::init();
    color_eyre::install().unwrap();

    /*
    args fetches the command-line arguments passed ot the binary program. the method
    nth(1) attempts to get the second argument, which is the path of the binary file.
    The unwrap method asserts that there is a second argument, and if not, the program
    will panic.

    let mut runner: Runner = Runner::new(&binary, false).unwrap(); declares a mutable
    variable runner of user-defined type Runner from tudelft_arm_qemu_runner crate
    developed by the course instructor, jdonszelmann.

    The constructor takes a reference to the binary, obtained from the command-line
    arguments, and a boolean value(false). The unwrap() method is called where it
    returns a Result type, which indicates that creating a new Runner instance could
    succeed or fail. The Runner runs a qemu instance, and get a handle to it’s UART
    input and output.
     */
    let binary = args().nth(1).unwrap();
    let mut runner: Runner = Runner::new(&binary, false).unwrap();


    // sleep(Duration::from_millis(10));
    // let a :u32 = 456765456;
    // let serialised = serialise(a);
    // runner.stream.write_all(serialised.as_slice())?;
    //
    // runner.stream.write_all(serialised.as_slice())?;
    //
    // runner.stream.write_all(serialised.as_slice())?;


    /*
    let mut is_first = true; is initialized to true for the infinite loop to
    print the program start-off statement on the terminal to familiarise the
    user only once before running the loop repeatedly.

    After printing the help message, is_first is set to false. This ensures
    that in subsequent iterations of the loop, this block of code will not
    be executed again.
     */
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

        /*
        let mut user_request declares a mutable variable user_request and
        initializes it with a new, empty String. This variable will be used
        to store the request provided by the user.

        io::stdin() is called to obtain the standard input handle for the
        program. This handle allows you to read from the standard input
        (typically the keyboard). The handle is stored in a variable named
        stdin.

        read_line method is called on the stdin handle to read a line of
        text from the user. The text is stored in the user_request string.
        The &mut user_request argument is a mutable reference to user_request,
        allowing read_line to modify its contents by appending the input.
        The expect method is used here as error handling. If read_line encounters
        an error (such as an I/O problem), instead of unwrapping the Result type
        directly, expect causes program to panic with the message "Invalid Request".
         */
        // Step 1 - Obtain Request from User
        let mut user_request = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut user_request).expect("Invalid Request");

        /*
        as_str method converts the String type of user_request to a string slice (&str).
        This is commonly done in Rust because many string-related methods operate on
        string slices rather than directly on String objects.

        trim()method removes any leading and trailing whitespace from the string slice.
        Whitespace here refers to spaces, tabs, and newline characters. This is important
        for cleaning up the user input, ensuring that any extra whitespace before the
        first word or after the last word does not affect the parsing process.

        .split(" ") method splits the trimmed string slice into an iterator of substrings
        wherever it finds the delimiter specified—in this case, a single space character
        (" "). The result is that if the user entered multiple words separated by spaces,
        each word would become a separate item in the iterator.

        .collect::<Vec<&str>>() method transforms the iterator of substrings into a collection.
        The type annotation ::<Vec<&str>> specifies that the collection should be a vector (Vec)
        of string slices (&str). This vector (tasks) now holds each separate word from the
        original user_request as its own element.
         */
        // Step 2 - Parse Tasks From User
        let tasks = user_request.as_str().trim().split(" ").collect::<Vec<&str>>();


        /*
        This portion of code defines a match statement to handle different requests entered
        by the user, for pattern-matching purposes.
         */
        // Step 3 - Match Task and Execute Task
        match user_request.as_str().trim() {
            // This arm matches the help command. If the user inputs -help, help, -h, or h,
            // the enclosed block of code will execute. The println! macro inside this arm
            // outputs a help message to the console, listing available commands and a
            // brief description of what they do.
            "-help" | "help" | "-h" | "h" => {
                println!("
                    Category    [command]       [message] or [ID]\n
                    Commands:   -help:          Display help\n
                                -w:             Move - Up\n
                                -a:             Move - Left\n
                                -s:             Move - Down\n
                                -d:             Move - Right\n
                                -t:             To Change Page View\n
                                -r:             Receive Total Number of Steps\n
                                -c:             Clear Step Count to Zero\n
                                -quit:          Quit Program\n
                     ");
                //stdout().lock().flush().unwrap();
            }
            // This arm matches the quit command. If the user inputs -quit, quit, -q, or q,
            // the break statement is executed, which will terminate the loop and thus end
            // the program.
            "-quit" | "quit" | "-q" | "q" => {
                break
            }
            // The underscore _ is a catch-all pattern that matches anything not previously
            // matched by the specific patterns in the match statement.
            _ => {
                match tasks[0] {
                    // Commands like -w, -a, -s, -d correspond to movement directions
                    // (up, left, down, and right, respectively).
                    // The -t command is presumably used to change the page view.
                    // The -c command is used to clear some count, probably a step counter.
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

                    /*
                    A DataFrame object with a payload of PayLoad::StepCountRequest is created.
                    This suggests that the payload is a request to retrieve the current step
                    count from some service or device.

                    The DataFrame object is then serialized using a serialise function. This
                    message i.e. entire slice of bytes is then sent to the expected recipient
                    over the stream connected to runner using the write_all method.

                    The program then pauses briefly with sleep(Duration::from_millis(5)), which
                    sleeps for 5 milliseconds so that it ensures stream of bytes is well-timed
                    for the embedded device to receive the message.

                    The code initializes recv_data as an empty vector to hold received bytes.
                    It sets up a buffer buf of 16 bytes, which is used to read incoming data from
                    the stream. The read method on runner.stream attempts to read data into the
                    buffer. The number of bytes actually read is stored in num_received.

                    A slice of the buffer from 0 to num_received is created, which contains only
                    the received data.A boolean variable tried is set to track whether an attempt
                    has been made to process the received data. The subsequent loop iterates
                    through the received bytes. For each byte: It pushes the byte onto recv_data.
                    If it encounters a byte with the value 0x00, this might indicate the end of
                    a message or data frame.

                    Within the inner conditional block, it attempts to deserialize the received
                    bytes back into a DataFrame using a function called deserialise. If
                    deserialization is successful and the payload is of type PayLoad::StepCount,
                    it prints out the current step count. If any errors occur during deserialization
                    or if the payload type does not match, it prints an error message.
                     */
                    "-r" | "r" => {
                        let msg = DataFrame{
                            payload: PayLoad::StepCountRequest,
                        };
                        let serialised = serialise(msg);
                        runner.stream.write_all(serialised.as_slice()).unwrap();

                        // wait for the message to get back.
                        sleep(Duration::from_millis(5));

                        let mut recv_data = Vec::<u8>::new();
                        // receive up to 16 bytes
                        let mut buf = [0u8; 16];
                        let num_received = runner.stream.read(&mut buf).unwrap();
                        // get the portion we actually received
                        let received = & mut buf[0..num_received];
                        let mut tried = false;
                        for single_byte in received.iter().as_slice() {
                            recv_data.push(*single_byte);
                            if *single_byte == 0x00 {
                                if let Some(data_frame) = deserialise(recv_data.as_mut_slice()){
                                    if let PayLoad::StepCount(stepcount) = data_frame.payload {
                                        println!("The current step count is: {}", stepcount);
                                    }
                                    else { println!("Error getting step count. Try again."); }
                                }
                                else { println!("Error getting step count. Try again."); }
                                recv_data.clear();
                                if tried == true {
                                    break;
                                }
                                else {
                                    tried = true;
                                }
                            }
                        }
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
