use std::process::Command;
use std::{thread, time::Duration};

use crate::process::ProcessInfo;
pub mod model;
pub mod process;
pub mod utils;
fn main() {
    let pid = std::process::id();
    let config = utils::get_config().unwrap();

    let handle = thread::spawn(move || {
        loop {
            let _ = Command::new("cmd").args(&["/C", "cls"]).status();
            let all_process = process::get_poc().unwrap();
            let process: Vec<&ProcessInfo> = all_process
                .iter()
                .filter(|process| {
                    !config.is_in_white_list(process.path())
                        && !process.is_system()
                        && !(process.pid() == pid)
                })
                .collect();
            let memory_kb = process
                .iter()
                .map(|process| process.memory_kb)
                .sum::<usize>();
            let private_kb = process
                .iter()
                .map(|process| process.private_memory_kb)
                .sum::<usize>();
            println!(
                "总进程数:{:<6}总内存占比{:<12}私有内存占比{:<12}",
                process.len(),
                memory_kb,
                private_kb
            );

            thread::sleep(Duration::from_secs(3));
        }
    });
    handle.join().unwrap();
}
