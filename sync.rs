//! ```cargo
//! [dependencies]
//! rusync = "*"
//! ```
use rusync;

fn main() {
    let console_info = rusync::ConsoleProgressInfo::new();
    // or any struct that implements the ProgressInfo trait

    let options = rusync::SyncOptions::default();

    println!("{:?}", console_info);
}
