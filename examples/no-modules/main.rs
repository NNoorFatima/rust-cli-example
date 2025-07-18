use std::process::Command;
use clap::{Arg, Command as ClapCommand};
use serde_json;

fn run_command(command: &str) -> String {
    let args: Vec<&str> = command.split(" ").collect();
    let output = Command::new(args[0])
        .args(&args[1..])
        .output()
        .expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.to_string()
}

fn run_lsblk(device: &str) -> serde_json::Value {
    let command = "lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT";
    let output = run_command(command);
    let devices: serde_json::Value = serde_json::from_str(&output).unwrap();
    let devices = devices["blockdevices"].as_array().unwrap();
    for parent in devices {
        if parent["name"] == device {
            return parent.clone();
        }
        if let Some(children) = parent["children"].as_array() {
            for child in children {
                if child["name"] == device {
                    return child.clone();
                }
            }
        }
    }
    panic!("Device not found");
}

fn main() {
    let matches = ClapCommand::new("lsblk")
        .version("0.0.1")
        .author("Alfredo Deza")
        .about("lsblk in Rust")
        .arg(
            Arg::new("device")
                .help("Device to query")
                .required(true)
                .index(1),
        )
        .get_matches();

    let device = matches.get_one::<String>("device").unwrap();
    let output = serde_json::to_string(&run_lsblk(&device)).unwrap();
    println!("{}", output);
}
