use os_info; // Operating system info
use std::time::Instant; // Date/Time

fn main() {
    let operating_sys_info = os_info::get();
    let now = Instant::now();
    
    loop_now();

    let elapsed_time = now.elapsed();

    let elapsed_nanoseconds = elapsed_time.as_nanos();
    let elapsed_seconds = elapsed_time.as_secs() % 60;
    let elapsed_minutes = (elapsed_time.as_secs() / 60) % 60;
    let elapsed_hours = (elapsed_time.as_secs() / 60) / 60;

    println!("Run time for Rust on {} is: {:02}:{:02}:{:02}.{}",
        operating_sys_info, elapsed_hours, elapsed_minutes, elapsed_seconds, elapsed_nanoseconds);
}

fn loop_now() {
    for i in 1..1000000001 {
        if i % 10000000 == 0 {
            println!("{i}");
        }
    }
}