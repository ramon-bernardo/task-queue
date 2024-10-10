use std::time::{Duration, SystemTime};
use std::{sync::mpsc, thread::sleep};

fn main() {
    // Create a channel to send and receive tasks.
    let (sender, receiver) = mpsc::channel::<Task>();

    // Send a task that does not expire.
    sender
        .send(Task::new_dont_expire(|| {
            println!("-> Executing task without expiration...");
        }))
        .ok();

    // Send a task that expires after 1 second.
    sender
        .send(Task::new(Duration::from_secs(1), || {
            println!("-> Starting task with a 1-second expiration. Sleeping for 2 seconds...");
            sleep(Duration::from_secs(2));
            println!("-> Completed task after sleep.");
        }))
        .ok();

    // Send a task that does not expire.
    sender
        .send(Task::new_dont_expire(|| {
            println!("-> Executing another task without expiration...");
        }))
        .ok();

    // Spawn a thread and clone the sender when needed inside the thread...
    std::thread::spawn({
        let sender = sender.clone();
        move || {
            sender
                .send(Task::new(Duration::from_secs(5), || {
                    println!("-> Executing task from another thread...");
                }))
                .ok();
        }
    })
    .join()
    .ok();

    // Send a task that will panic after 5 seconds.
    sender
        .send(Task::new(Duration::from_secs(5), || {
            println!("-> Task will panic in 5 seconds!");
            panic!();
        }))
        .ok();

    // Spawn a thread to process tasks from the receiver.
    std::thread::spawn(move || {
        while let Ok(task) = receiver.recv() {
            // Only run the task if it hasn't expired.
            if !task.has_expired() {
                task.run();
            }
        }
    })
    .join()
    .ok();
}

/// A struct representing a task that can be executed.
pub struct Task {
    /// The expiration time of the task, or `None` if the task does not expire.
    expires_at: Option<SystemTime>,
    /// The function to be executed when the task is run.
    func: fn(),
}

impl Task {
    /// Creates a new task that expires after the specified duration.
    pub fn new(expires_at: Duration, func: fn()) -> Self {
        Self {
            expires_at: Some(SystemTime::now() + expires_at),
            func,
        }
    }

    /// Creates a new task that does not expire.
    pub fn new_dont_expire(func: fn()) -> Self {
        Self {
            expires_at: None,
            func,
        }
    }

    /// Checks if the task has expired.
    pub fn has_expired(&self) -> bool {
        self.expires_at
            .map_or(false, |expires_at| expires_at < SystemTime::now())
    }

    /// Executes the task.
    pub fn run(self) {
        (self.func)()
    }
}
