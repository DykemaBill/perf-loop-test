use os_info; // Operating system info
use std::time::Instant; // Date/Time

fn main() {
    // Get the OS
    let operating_sys_info = os_info::get();
    // Get the start time
    let now = Instant::now();
    
    // Loop just for timed comparison
    loop_now();

    // Gather times
    let elapsed_time = now.elapsed();
    let elapsed_secdecimals = elapsed_time.as_millis() % 1000;
    let elapsed_seconds = elapsed_time.as_secs() % 60;
    let elapsed_minutes = (elapsed_time.as_secs() / 60) % 60;
    let elapsed_hours = (elapsed_time.as_secs() / 60) / 60;

    // Show the results
    println!("Run time for Rust on {} is: {:02}:{:02}:{:02}.{}",
        operating_sys_info, elapsed_hours, elapsed_minutes,
        elapsed_seconds, elapsed_secdecimals);
}

fn loop_now() {
    for i in 1..1000000001 {
        if i % 10000000 == 0 {
            println!("{i}");
        }
    }
}