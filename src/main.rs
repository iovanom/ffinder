mod info;
mod settings;

use settings::Settings;
use std::collections::VecDeque;
use std::io::prelude::*;

fn main() {
    let settings = Settings::new();
    let mut dirs = VecDeque::new();
    dirs.push_back(settings.start_dir);
    while !dirs.is_empty() {
        let dir = dirs.pop_front().unwrap();
        if let Err(_) = display(dir.to_str().unwrap()) {
            std::process::exit(0);
        };
        match dir.read_dir() {
            Ok(target) => {
                for entry in target {
                    let path = entry.unwrap().path();
                    if path.is_dir() {
                        dirs.push_back(path);
                    }
                }
            }
            _ => {}
        }
    }
}

fn display(dir: &str) -> std::io::Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", dir)
}
