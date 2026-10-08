use std::{process::Command};

pub fn start(command: &str) -> i32 {
    #[cfg(windows)]
    {
        let mut args: Vec<String> = vec!["/C".to_string()];
        let mut process = Command::new("cmd");
        for arg in command.split_whitespace() {
            args.push(arg.to_string());
        }
        process.args(args);

        return match process.status() {
            Ok(status) => status.code().unwrap_or(-1),
            Err(_) => -1,
        }
    }

    #[cfg(not(windows))]
    {
        let mut args: Vec<String> = vec!["-c".to_string()];
        let mut process = Command::new("sh");
        process.arg(command);

        return match process.status() {
            Ok(status) => status.code().unwrap_or(-1),
            Err(_) => -1,
        }
    }
}
