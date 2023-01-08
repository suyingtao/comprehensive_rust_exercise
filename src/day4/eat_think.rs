use std::sync::mpsc::{self, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: SyncSender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        println!("{} is trying to eat", &self.name);
        let left = self.left_fork.lock().unwrap();
        let right = self.right_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    let (tx, rx) = mpsc::sync_channel(10);
    let n = PHILOSOPHERS.len();
    let forks = (0..n)
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    for i in 0..n {
        let mut left_fork = forks[i].clone();
        let mut right_fork = forks[(i + 1) % forks.len()].clone();
        if i == forks.len() - 1 {
            std::mem::swap(&mut left_fork, &mut right_fork);
        }
        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            left_fork,
            right_fork,
            thoughts: tx.clone(),
        };
        
        thread::spawn(move || {
            for _ in 0..10 {
                philosopher.eat();
                philosopher.think();
            }
        });
    }
    drop(tx);
    for thought in rx {
        println!("{}", thought);
    }

}
