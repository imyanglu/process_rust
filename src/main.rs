pub mod model;
pub mod process;
pub mod utils;
fn main() {
    let pid = std::process::id();
    let process_infos = process::get_poc().unwrap();
    process_infos
        .iter()
        .filter(|&process| process.is_system())
        .for_each(|p| {
            println!("{}", p);
        });
}
