use std::{env, path};
use std::fs::File;
use std::io::{Read};

use crate::parser::parse_processes;

mod parser;
mod process;

const VERSION: &str = "1.0";

// runc [GROUP/-v] [CONFIG_FILE]
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let config_name = arguments
        .get(2)
        .map(String::as_str)
        .unwrap_or("runinfo.command");


    let arguments: Vec<String> = env::args().collect();

    match arguments.get(1).map(String::as_str) {
        Some("-v") => {
            println!("\x1b[1m    RunC version {} \x1b[0m", &VERSION.to_string());
            return;
        }
        None | Some("-h") => {
            println!(
                "\nHELP:\n \x1b[1m    Command Example: runc [GROUP/-v/-h] [CONFIG_FILE]\x1b[0m\n"
            );
            return;
        }
        _ => {}
    }

    println!("\x1b[1mRunC: Running group '{}'\x1b[0m", &arguments[1]);

    let config_path = path::Path::new(config_name);
    if !config_path.exists() {
        println!("RunC: 'runinfo.command' not exists in this directory");
    }

    let mut config_file = match File::open(&config_path) {
        Ok(config_file) => config_file,
        Err(_) => { eprintln!("RunC: Error while opening config file"); return; }
    };
    let mut content = String::new();
    if config_file.read_to_string(&mut content).is_err() {
        eprintln!("RunC: Error while reading config file");
        return;
    }

    let processes: Vec<String> = parse_processes(content, arguments[1].to_string());

    for proc in processes {
        process::start(&proc);
    }
}
