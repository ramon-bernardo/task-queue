# Task Queue

This is a simple task queue implemented in Rust. It allows you to create tasks that can either expire after a specified duration or run indefinitely. The application uses channels for communication between threads and handles task execution in a separate thread.

The `Task` struct represents a task that can be executed. It includes the following methods:
- `new`: Creates a new task that expires after a specified duration.
- `new_dont_expire`: Creates a new task that does not expire.
- `has_expired`: Checks if the task has expired.
- `run`: Executes the task.

## Program Output
```
-> Executing task without expiration...
-> Starting task with a 1-second expiration. Sleeping for 2 seconds...
-> Completed task after sleep.
-> Executing another task without expiration...
-> Executing task from another thread...
-> Task will panic in 5 seconds!
thread '<unnamed>' panicked at src/main.rs:49:13
```

## Concept
This project is based on the concept of the [Dispatcher Thread](https://github.com/otland/forgottenserver/blob/74b61cda4a74186f1920e9dd152c13b953582b47/src/tasks.h#L47-L68) from [The Forgotten Server](https://github.com/otland/forgottenserver), replacing the use of mutex with an [Multi-producer, single-consumer](https://doc.rust-lang.org/std/sync/mpsc/index.html) (lock-free) channel in Rust to optimize message exchange between threads.