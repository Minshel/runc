use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::{collections::HashMap, fs};

use crate::parser::parse_processes;

mod parser;
mod process;

// runc [GROUP] [CONFIG_FILE]
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let config_name = arguments
        .get(2)
        .map(String::as_str)
        .unwrap_or("runinfo.command");

    println!("\x1b[1mRunC: Running group '{}'\x1b[0m", &arguments[1]);

    let exe_dir = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let config_path = exe_dir.join(&config_name);
    if !config_path.exists() {
        let mut config_file = match File::create(&config_path) {
            Ok(config_file) => config_file,
            Err(_) => { eprintln!("RunC: Error while config file creation"); return; }
        };
        config_file.write("[main]\n".as_bytes());
    }

    let mut config_file = match File::open(&config_path) {
        Ok(config_file) => config_file,
        Err(_) => { eprintln!("RunC: Error while opening config file"); return; }
    };
    let mut content = String::new();
    config_file.read_to_string(&mut content);

    let processes: Vec<String> = parse_processes(content, arguments[1].to_string());

    for proc in processes {
        let status: i32 = process::start(&proc);
        println!("{} started with code {}", proc, status);
    }
}
