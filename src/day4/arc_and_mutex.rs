use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let mutex_v = Mutex::new(vec![10, 20, 30]);
    let arc_v = Arc::new(mutex_v);
    let arc_v_cloned = arc_v.clone();
    let handle = thread::spawn(move || {
        arc_v_cloned.lock().unwrap().push(10);
    });
    arc_v.lock().unwrap().push(1000);

    handle.join().unwrap();
    println!("v: {arc_v:?}");
}