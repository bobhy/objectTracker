#![warn(missing_docs)]

//! Messages we can exchange through [`mpsc::channel`] with tasks.
//! Hand-crafted as a learning exercise,
//! but I bet there's lots to learn from studying https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
//!
//! This module enumerates all possible messsage types here [`TaskOps`].  A more flexible
//! approach might be to implement the op code and data payload together with the task code all in
//! a per-task module.
//!
//! But this is how it works right now...
//!
//! # Examples
//!
//! ```rust
//! use std::sync::mpsc;
//! use std::thread;
//! use super::{TaskOp, TaskMessage};
//!
//! #[derive(Debug, Copy, Clone, PartialEq)]
//! struct ArbPayload {
//!     f1: usize,
//! }
//!
//! let send_payload = ArbPayload{f1: 27};
//!
//! let (tx, rx) = mpsc::channel::<TaskMessage<ArbPayload>>();
//! let task_handle = thread::spawn(move || {
//!     tx.send(TaskMessage::new(TaskOp::Echo, &send_payload)).unwrap();
//! } );
//!
//! let receive_msg = rx.recv().unwrap();
//!
//! assert_eq!(receive_msg.payload.f1, 27);
//! assert_eq!(receive_msg, TaskMessage::new(TaskOp::Echo, &send_payload));
//!
//! task_handle.join().unwrap();
//! ```
//! Commands we can send to tasks
//!

use std::fmt;
use derive_more::{Display};

#[derive(Debug, PartialEq)]

/// Messages we can exchange with tasks
pub enum TaskMessage {
    /// Shutdown (cooperative)
    ShutdownRequest,
    /// Shutdown response
    ShutdownResponse,
    /// Echo request
    EchoRequest(String),
    /// Echo response
    EchoResponse(String),
    /// Move the servo request (fraction of travel: 0-1.0, 0.5 is neutral)
    ServoRequest(f32),
    /// Response o move the servo: fraction the servo is "currently" at (might still be in transit to desired position)
    ServoResponse(f32),
    /// Frame grab
    GrabRequest,
    /// Frame grab response: a list of "results"
    GrabResponse(Vec<i32>)
}


#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_TaskMessage_constr_access() {
        // The only way to access interior values of the message is with a pattern??
        //future -- maybe write some getter methods on TaskMessage ???
        let a = TaskMessage::EchoRequest(String::from("Hello"));

        if let TaskMessage::EchoRequest(foo) = a {
            assert!(foo == "Hello")
        };

        let b = TaskMessage::EchoResponse(String::from("garbage"));

        // whottahack! since I want to use b in later test, I must think ahead and use a reference here.
        if let TaskMessage::EchoResponse(bar) = &b {
            assert!(&bar == &"garbage")
        };

        let c = TaskMessage::ShutdownRequest;

           // let a = (String::from("Hello"), );

        //assert!("Hello" == a);
        //assert!(b == "garbage");

        match b {
            TaskMessage::EchoRequest(q) => assert!(&q == "Hello"),
            TaskMessage::EchoResponse(q) => assert!(&q == "garbage"),
            _ => assert!(1 == 0, "wrong branch in match")
        }
    }


    #[test]
    fn enum_display_test() {
        #[derive(Display, Debug)]
        enum Foo {
            Var1,
            Argle,
            Blort
        }

        let f1 = Foo::Var1;
        let f2 = Foo::Blort;

        assert!(format!("{:?}", f1) == "Var1");
        assert!(format!("{}", f2) == "Blort");
    }
}
