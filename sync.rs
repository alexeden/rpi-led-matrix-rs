//! rsync
//!     -ahPr
//!     --rsh={command}
//!     --exclude=.git
//!     --exclude=.DS_Store
//!     --exclude=node_modules
//!     --exclude=build
//!     --exclude=package-lock.json
//!     --exclude=.vscode
//!     --exclude=dist
//!     /Users/alexeden/code/rpi-led-matrix
//!     pi@192.168.1.201:~


fn main() {
    let console_info = rusync::ConsoleProgressInfo::new();
    // or any struct that implements the ProgressInfo trait

    let options = rusync::SyncOptions::default();

    println!("{:?}", console_info);
}
