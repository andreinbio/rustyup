use std::process::Command;
use std::fs;

pub fn is_new_update() -> bool {
    false
}

pub fn start_update() {
    let exe_path = fs::canonicalize("./crates/update/target/release/update.exe");
    // dbg!(exe_path);
    Command::new(exe_path.unwrap())
        .spawn()
        .expect("failed to run update.exe");
}