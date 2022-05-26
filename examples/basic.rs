fn main() {
    let join_handle = notify_thread::spawn(|ctx| {
        while !ctx.notified() {
            println!("Looping");

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    std::thread::sleep(std::time::Duration::from_millis(1000));

    join_handle.notify();
    join_handle.join().unwrap();
}
