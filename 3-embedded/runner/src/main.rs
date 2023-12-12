use std::env::args;
use std::io::{stdout, Read, Write};
use std::thread::sleep;
use std::time::Duration;

use tudelft_arm_qemu_runner::Runner;
use common_lib::{deserialise, serialise};

fn main() -> color_eyre::Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let binary = args().nth(1).unwrap();
    let mut runner: Runner = Runner::new(&binary, false)?;
    let mut recv_data = Vec::<u8>::new();
    // receive up to 256 bytes
    let mut buf = [0u8; 256];

    loop{
        //sleep(Duration::from_millis(10));
        let a :u32 = 456765456;
        let serialised = serialise(a);
        runner.stream.write_all(serialised.as_slice())?;
    }

    loop {
        sleep(Duration::from_millis(20));

        let num_received = runner.stream.read(&mut buf)?;
        // get the portion we actually received
        let received = & mut buf[0..num_received];

        for single_byte in received.iter().as_slice() {
            recv_data.push(*single_byte);
            if *single_byte == 0x00 {
                runner.stream.write_all(recv_data.as_slice())?;
                if let Some(rec) = deserialise(recv_data.as_mut_slice()){
                    if rec == 456765456 {
                        print!(" OK "); stdout().lock().flush().unwrap();
                    }
                    else {
                        print!(" fail1 "); stdout().lock().flush().unwrap();
                    }
                }
                else {
                    print!(" fail2 "); stdout().lock().flush().unwrap();
                }
                recv_data.clear();
            }
        }
        // print it (without newline)


        // flush to show what's printed without having to print a newline
        stdout().lock().flush().unwrap();

        // send back the bytes to the Stellaris board
        // runner.stream.write_all(serialise(456765456).as_slice())?;
        let a :u32 = 456765456;
        let serialised = serialise(a);
        runner.stream.write_all(serialised.as_slice())?;
    }
}
