// use std::process::Command;
// use std::time::Duration;
use psutil::process::processes;
use std::thread;
use std::time::Duration;

fn main() {
    use std::process::Command;
    // println!("Hello, world!");
    let mut the_process = Command::new("sh")
        .arg("-c")
        .arg("echo hello;sleep 100;echo world")
        .spawn()
        .ok()
        .expect("failed to execute process");
    println!("The PID is: {}", the_process.id());
    let processes = processes().unwrap();

    thread::sleep(Duration::from_secs(1));
    for p in processes {
        let mut p = p.unwrap();
        if (p.pid() == the_process.id()) {
            println!(
                "{:>6} {:>4} {:?} {:.100}",
                p.pid(),
                p.cpu_percent().unwrap(),
                p.memory_info(),
                p.name().unwrap()
            );
        }
    }

    match the_process.wait() {
        Ok(status) => println!("Finished, status of {}", status),
        Err(e) => println!("Failed, error: {}", e),
    }
    // println!("status: {:?}", output);
    // let mut echo_hello = Command::new("sh");
    // echo_hello.arg("-c").arg("echo hello");
    // echo_hello.spawn().expect("failed to execute process");
}
