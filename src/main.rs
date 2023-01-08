// use std::sync::mpsc::{self, SyncSender};
use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;

#[derive(Debug)]
struct Fork;

fn main() {
    let f = Arc::new(Mutex::new(Fork));
    let a = f.lock().unwrap();
    {
        println!("{a:?}");
    }
    let b = f.lock().unwrap();
    println!("{b:?}");
}
