use clap::{Arg, Command};
use std::env;
use std::process::Command as SystemCommand;

fn main() {
    let matches = Command::new("VS Code Opener")
        .version("1.0")
        .author("Your Name")
        .about("Opens files or directories in VS Code")
        .arg(
            Arg::new("new-window")
                .long("new")
                .value_name("FILE_OR_DIR")
                .help("Open in a new window")
                .action(clap::ArgAction::Set),
        ) // Use ArgAction::Set for clap 4.x
        .arg(
            Arg::new("reuse-window")
                .long("reuse")
                .value_name("FILE_OR_DIR")
                .help("Reuse an existing window")
                .action(clap::ArgAction::Set),
        ) // Use ArgAction::Set for clap 4.x
        .get_matches();

    let path = if let Some(path) = matches.get_one::<String>("new-window") {
        path
    } else if let Some(path) = matches.get_one::<String>("reuse-window") {
        path
    } else {
        eprintln!("Please specify a path with --new or --reuse option.");
        return;
    };

    // Check which VS Code version to use
    let vscode_command = if command_exists("code") {
        "code"
    } else if command_exists("code-insiders") {
        "code-insiders"
    } else {
        eprintln!("Neither 'code' nor 'code-insiders' is installed.");
        return;
    };

    let mut command = SystemCommand::new(vscode_command);

    if matches.get_one::<String>("new-window").is_some() {
        command.arg("--new-window");
    } else if matches.get_one::<String>("reuse-window").is_some() {
        command.arg("--reuse-window");
    }

    let status = if path == "." {
        command
            .current_dir(env::current_dir().unwrap())
            .arg(".")
            .status()
    } else {
        command.arg(path).status()
    };

    match status {
        Ok(status) if status.success() => {
            println!("Opened '{}' successfully with '{}'.", path, vscode_command)
        }
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
