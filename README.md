# notify-thread

A simple wrapper for threads that allows you to notify the thread that something has happened.

```rust
fn main() {
    // Spawn a new thread that waits until notified
    let join_handle = easy_thread::spawn(|ctx| {
        while !ctx.notified() {
            println!("Looping");

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    std::thread::sleep(std::time::Duration::from_millis(1000));

    // Notify the thread
    join_handle.notify();

    join_handle.join().unwrap();
}
```