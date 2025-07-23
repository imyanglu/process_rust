use std::process::Command;
use std::{thread, time::Duration};

use crate::process::ProcessInfo;
use crate::utils::convert_kb;
pub mod model;
pub mod perfect;
pub mod process;
pub mod utils;
fn main() {
    let pid = std::process::id();
    let config = utils::get_config().unwrap();

    let handle = thread::spawn(move || {
        loop {
            let _ = Command::new("cmd").args(&["/C", "cls"]).status();
            let all_process = process::get_poc().unwrap();
            let mut process: Vec<&ProcessInfo> = all_process
                .iter()
                .filter(|process| {
                    !config.is_in_white_list(process.path())
                        && !process.is_system()
                        && !(process.pid() == pid)
                })
                .collect();
            process.sort_by(|a, b| {
                let res = a.name.cmp(&b.name);
                if res == std::cmp::Ordering::Equal {
                    return a.memory_kb.cmp(&b.memory_kb);
                }
                return res;
            });
            let memory_kb = process
                .iter()
                .map(|process| process.memory_kb)
                .sum::<usize>();
            let private_kb = process
                .iter()
                .map(|process| process.private_memory_kb)
                .sum::<usize>();
            let memory_unit = convert_kb(memory_kb);
            let private_unit = convert_kb(private_kb);
            let mem_str = format!("{}G-{}M-{}K", memory_unit.0, memory_unit.1, memory_unit.2);
            let private_str = format!(
                "{}G-{}M-{}K",
                private_unit.0, private_unit.1, private_unit.2
            );
            println!(
                "总进程数:{:<6}总内存占比{:<24}私有内存占比{:<24}",
                process.len(),
                mem_str,
                private_str
            );
            process.iter().for_each(|process| {
                let memory_unit = convert_kb(process.memory_kb);
                let private_unit = convert_kb(process.private_memory_kb);
                let mem_str = format!("{}G-{}M-{}K", memory_unit.0, memory_unit.1, memory_unit.2);
                let private_str = format!(
                    "{}G-{}M-{}K",
                    private_unit.0, private_unit.1, private_unit.2
                );
                println!("{:<30} {:<44} {:<24}", process.name, mem_str, private_str);
            });

            thread::sleep(Duration::from_secs(3));
        }
    });
    handle.join().unwrap();
}
