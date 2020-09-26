#[macro_use]
extern crate serde;

use std::io;
use std::path::PathBuf;
use std::{collections::HashMap, fs::File};
use std::{env::current_dir, process::ExitStatus};
use std::{io::Read, process::Command};

const SYNC_CONFIG_FILENAME: &str = "sync.toml";

#[derive(Debug, Deserialize)]
struct SyncConfig {
    #[serde(skip)]
    path: PathBuf,
    config: Config,
    remote: HashMap<String, Remote>,
}

#[derive(Debug, Deserialize)]
struct Config {
    command: String,
    exclude: Vec<String>,
    selected_remote: String,
}

#[derive(Debug, Deserialize)]
struct Remote {
    dest_dir: String,
    hostname: String,
    username: String,
}

impl ToString for Remote {
    fn to_string(&self) -> String {
        format!("{}@{}:{}", self.username, self.hostname, self.dest_dir)
    }
}

impl SyncConfig {
    pub fn from_path(path: &PathBuf) -> Result<Self, io::Error> {
        File::open(path)
            .and_then(|mut file| {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).map(|_| buffer)
            })
            .and_then(|contents| {
                toml::from_str::<SyncConfig>(&contents)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            })
            .map(|mut config| {
                config.path = path.to_owned();
                config
            })
    }

    pub fn selected_remote(&self) -> &Remote {
        self.remote
            .get(&self.config.selected_remote)
            .expect(&format!(
                "Selected remote \"{}\" does not match any defined remotes.",
                self.config.selected_remote
            ))
    }

    pub fn sync(&self) -> ExitStatus {
        let mut cmd = Command::new("rsync");
        let cmd = cmd.env("TERM", "xterm-256color").arg("-ahr").args(&[
            "--progress",
            "--rsh",
            &self.config.command,
        ]);

        &self.config.exclude.iter().for_each(|ex| {
            cmd.args(&["--exclude", ex]);
        });

        // Source
        cmd.arg(self.path.parent().unwrap());

        // Destination
        cmd.arg(self.selected_remote().to_string());

        cmd.status().expect("Failed to run command.")
    }
}

fn main() {
    let config_path = find_file(SYNC_CONFIG_FILENAME)
        .expect("A config file named sync.toml exists in the current or ancestor directory.");

    let status = SyncConfig::from_path(&config_path).unwrap().sync();
    println!("Done! Exit status: {:?}", status);
}

fn find_file(file_name: &str) -> Option<PathBuf> {
    current_dir()
        .unwrap()
        .join(PathBuf::from(file_name))
        .ancestors()
        .find_map(|parent| {
            let path = parent.join(PathBuf::from(file_name));
            if path.exists() {
                Some(path)
            } else {
                None
            }
        })
}
