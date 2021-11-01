#![warn(missing_docs)]
//! Tutorial program to play with object recognition
//!
//! Also is a tutorial on Rust programming and tools.

mod tasks;

use ctrlc;
use mt_logger::*;
use std::sync::mpsc;
use std::thread;
use std::time;


/// Main
/// CLI to activate the program.
///
fn main() {
    // use multithreaded logging in all threads, to avoid garbling output.  Order between lines logged in multiple threads is not guaranteed.
    mt_new!(None, Level::Info, OutputStream::StdOut);

    mt_log!(Level::Info, "Main startup");

    // main listens on single channel for results from all the subtasks.
    // the subtasks each get a clone of the sending side of the channel to post on.
    /*
    let (control_tx, control_rx) = mpsc::sync_channel::<TaskMessage<String>>(10);

    // Arm the control-c handler
    let cc_control_tx = control_tx.clone();

    let _ = ctrlc::set_handler(move || {
        cc_control_tx
            .send(TaskMessage::<String>::from_str(TaskOp::Shutdown, &""))
            .unwrap();
    })
    .unwrap();

    // Start the echo task running.
    let (echo_tx, echo_rx) = mpsc::channel::<TaskMessage<String>>();
    let echo_control_tx = control_tx.clone();

    let _ = thread::spawn(move || do_echo(echo_control_tx, echo_rx));
    echo_tx
        .send(TaskMessage::from_str(TaskOp::Echo, "Polo"))
        .unwrap(); // first one's free.

    loop {
        let completion = control_rx.recv().unwrap();

        mt_log!(Level::Info, "Received: {}", completion);

        match completion.operation {
            // exit with some grace
            TaskOp::Shutdown => {
                break;
            }
            // Give echo task something to do
            TaskOp::Echo => {
                let f = echo_tx.send(TaskMessage::from_str(TaskOp::Echo, "Marco"));
                let _ = match f {
                    Ok(v) => v,
                    Err(e) => mt_log!(Level::Error, "Error {} sending to echo task.", e),
                };
                // give echo task another thing to do
            }
            other => {
                panic!("Unsupported operation: {}", other)
            }
        };
    }
    */
    mt_log!(Level::Info, "End of main, goodbye.");
    mt_flush!().unwrap();
}

/*
/// The echo task: get your request, delay a bit, then echo mangled version of input
///
/// Note there doesn't seem to be a supported way to kill a task outright.
/// So, to get the task to cooperatively shut down, send it a shutdown message and wait for it to see it.
pub fn do_echo(tx: mpsc::SyncSender<TaskMessage<String>>, rx: mpsc::Receiver<TaskMessage<String>>) {
    loop {
        let request = rx.recv().unwrap();
        let payload: String = match request.operation {
            TaskOp::Shutdown => {
                break;
            }
            TaskOp::Echo => request.payload.to_string(),
            other => {
                panic!("Echo task got unexpected command: {}", other);
            }
        };
        // if control gets here, process payload
        assert_eq!(request.operation, TaskOp::Echo);

        thread::sleep(time::Duration::from_millis(1000));

        let rsp = TaskMessage::from_str(TaskOp::Echo, &payload.to_ascii_uppercase());
        tx.send(rsp).unwrap();
    }
}

#[cfg(test)]

/// tests
///
/// Found at the bottom of relevant source files, untill we outgrow that.
pub mod tests {
    use super::*;

    #[test]
    fn main_args() {
        assert_eq!(1, 1);
    }

    #[test]
    fn main_fail() {
        panic!("intentional fail");
    }
    /// internal test -- captured by documentation
    /// # Examples
    /// ```
    /// assert!(Ok(11, int_fun(22))
    /// ```
    ///
    pub fn int_fun(
        // an argument for argument's sake
        a: u32,
    ) -> Result<u32, ()> {
        Ok(1)
    }


}
*/
