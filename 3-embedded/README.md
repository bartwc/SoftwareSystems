# Software Systems - Assignment 3: Embedded Systems

In this assignment you are going to build a small program running inside an emulator which emulates a Stellaris LM3S6965 microcontroller with an ARM M3 core.
You will be building a kind of GPS step counter that records changes in position, counts steps and displays some information in a graphical interface.
You will design a protocol to communicate with your system and implement a UART driver which provides a channel to communicate over.

## Installation Requirements
> Install and Configure the Lab Setup [here](https://cese.pages.ewi.tudelft.nl/software-systems/part-1/assignments/es.html).

### Part 1: UART
1. You must create a safe interface (between UART driver and Protocol) through which to send and receive UART messages [uart.rs](embedded-code%2Fsrc%2Fuart.rs)
2. You must provide a safe abstraction for global mutable state (Mutex) file - [mutex.rs](embedded-code%2Fsrc%2Fmutex.rs)
   * You need to somehow create a way of preventing two threads executing at once. Of course there are no threads in the system you are implementing the driver for, but concurrency is still an issue.
   * This may need unsafe code, you need to add an explanation on why your implementation is still sound.
3. You must provide buffering such that long messages can be sent and received. - [ringbuffer.rs](embedded-code%2Fsrc%2Fringbuffer.rs)
   * The buffer must be fixed-size, and must be sized appropriately for the messages that are sent over them.
   * If the buffer fills up, the program must not crash.

### Part 2: Protocol / Communication
While the emulator runs, it should keep track of a "location" and amount of steps.

4. You should be able to record changes in position (dx/dy in meters) and remember that state in the program.
5. You should be able to record "a step" that is also kept track by the program running in the emulator
6. You should be able to send a message to the program to change the currently viewed page
7. You should use checksums to verify the integrity of your messages
8. If messages are dropped, or corrupted, you should make sure that your system can continue reading future messages. Test this.
You must use the serde library

### Part 3: Interface
In this section, some choices are left up to you. You can be a bit creative here.

10. The Graphical Interface must contain two views
    * A map of where the user has walked, 1 pixel = 1 meter. You can assume the user won't walk far enough to leave the screen boundaries.
    * A simple counter view which shows total number of steps taken
11. The runner should get a simple interactive interface in which you can simulate sending messages to the system by typing commands. [main.rs](runner%2Fsrc%2Fmain.rs) in /runner/src

* You should be to send messages corresponding to the following actions:
    * A single step being taken, and the corresponding dx/dy movement.
    * A help page (for us) so we know what to type
    * A command to request the current total number of steps recorded by the Stellaris board
    * A reset command, that resets the state of the Stellaris board to its initial state, so 0 steps taken.

## Help to Run Terminal
Run ```cargo run -- release``` to run program.
```
 -----------------------------------------------------------------
 |   Hi User! Key In Request to Initialize! Enter -help for Help |
 -----------------------------------------------------------------
```
Run -help for full list of available requests.
```
Category    [request]       [task]\n
Commands:   -help:          Display help\n
            -w:             Move - Up\n
            -a:             Move - Left\n
            -s:             Move - Down\n
            -d:             Move - Right\n
            -t:             Transmit Message to Change Page View\n
            -r:             Receive Total Number of Steps\n
            -c:             Clear Step Count to Zero\n
            -quit:          Quit Program\n
```
Run -quit to end the instance of the program.
```
 -----------------------------------------------------------------
|             Bye User!  *** Program Has Quit ***                |
------------------------------------------------------------------
```

## Authors
[@Zhengtao Huang (5833469, zhengtaohuang)]()<br>
[@Barry Tee Wei Cong (5662834, btee)]()

## Acknowledgments
* [Assignment 3 - Embedded Systems](https://cese.pages.ewi.tudelft.nl/software-systems/part-1/assignments/es.html)