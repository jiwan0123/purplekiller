use sysinfo::{System, ProcessExt, SystemExt, Pid, Signal};

fn main() {
    let s = System::new_all();

    for process in s.processes_by_name("Purple") {
        if let Some(p) = s.process(Pid::from(process.pid().to_string().parse::<usize>().unwrap())) {
            if p.kill_with(Signal::Kill).is_none() {
                eprintln!("this supported signal is not supported on your platform")
            }
        }
    }

    for process in s.processes_by_name("purple") {
        if let Some(p) = s.process(Pid::from(process.pid().to_string().parse::<usize>().unwrap())) {
            if p.kill_with(Signal::Kill).is_none() {
                eprintln!("this supported signal is not supported on your platform")
            }
        }
    }
}
