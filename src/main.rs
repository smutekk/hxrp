use discord_presence::{Client, Event};
use regex::Regex;
use std::process::{Command, ExitStatus};

fn fetch_running(name: &str) -> ExitStatus {
    let pCommand = Command::new("pgrep")
        .arg("-x")
        .arg(name)
        .output()
        .expect("Failed process command");

    let fileCommand = Command::new("pgrep")
        .arg("a")
        .arg(name)
        .output()
        .expect("Failed file command");

    let fileCommandOutput = String::from_utf8(fileCommand.stdout);

    let re = Regex::new(r"^[^ ]*\s+[^ ]*\s+"e).unwrap();
    let fileName = re.replace(fileCommandOutput, "");

    pCommand.status
}

fn main() {
    if fetch_running("helix").success() {
        println!("Helix running!");
    } else {
        println!("Helix not running.");
    }
}
