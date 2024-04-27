use clap::{Command, Arg};
use std::process::Command as SystemCommand;
use std::env;

fn main() {
    let matches = Command::new("VS Code Opener")
        .version("1.0")
        .author("Your Name")
        .about("Opens files or directories in VS Code")
        .arg(Arg::new("path")
            .help("The file or directory to open, use '.' to open the current directory")
            .required(true)
            .index(1))
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();

    // Check which VS Code version to use
    let vscode_command = if command_exists("code") {
        "code"
    } else if command_exists("code-insiders") {
        "code-insiders"
    } else {
        eprintln!("Neither 'code' nor 'code-insiders' is installed.");
        return;
    };

    let status = if path == "." {
        SystemCommand::new(vscode_command)
            .current_dir(env::current_dir().unwrap())
            .arg(".")
            .status()
    } else {
        SystemCommand::new(vscode_command)
            .arg(path)
            .status()
    };

    match status {
        Ok(status) if status.success() => println!("Opened '{}' successfully with '{}'.", path, vscode_command),
        Ok(_) => eprintln!("VS Code exited with an error."),
        Err(e) => eprintln!("Failed to execute VS Code: {}", e),
    }
}

/// Checks if a command exists by trying to get its version.
fn command_exists(command: &str) -> bool {
    SystemCommand::new(command)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
