// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = Arc::clone(&status);

    thread::spawn(move || {
        for _ in 0..10 { // NOTE: It going up to 11? That would be ridiculous!
            thread::sleep(Duration::from_millis(250));
            let mut s = status_shared.lock().unwrap();
            s.jobs_completed += 1;
        }
    });

    loop {
        thread::sleep(Duration::from_millis(500));
        let s = status.lock().unwrap();

        if s.jobs_completed < 10 {
            println!("Waiting... {} so far", s.jobs_completed);
        } else {
            println!{"At {} we are done!", s.jobs_completed}
            break;
        }
    }
}
