I don't understand why you aren't provided it in the read me. 

# VS Code Opener

## Description
VS Code Opener is a simple, efficient command-line tool developed in Rust, designed to open files or directories directly in Visual Studio Code. This utility supports both `code` and `code-insiders` executables, intelligently defaulting to `code` if both are available.

## Features
- **File Opening:** Open any specific file directly in Visual Studio Code.
- **Directory Opening:** Open directories in Visual Studio Code, including support for the current directory via `.`.
- **Window Management:** Choose to open files or directories in a new window or reuse an existing one.
- **Executable Detection:** Automatically detects and utilizes `code` or `code-insiders`, with a preference for `code` if both are installed.

Open a file or directory in new VS code window
```bash
vsc --new <location>
```

Reuse an existing VS Code window to open a file or directory
```bash
vsc --reuse <location>
```

Open the current directory in a new VS Code window
```bash
vsc --new .
```

Reuse an existing VS Code window to add the current directory
```bash
vsc --reuse .
```

## Installation

### Prerequisites
Ensure the following are installed:
- **Rust and Cargo:** Follow the [Rust installation guide](https://www.rust-lang.org/tools/install) to set up Rust and Cargo on your system.
- **Visual Studio Code:** Both standard and Insiders editions are supported. The `code` command must be available in your system's PATH.

### Installing Visual Studio Code
To use the VS Code Opener, Visual Studio Code needs to be installed, and the `code` command must be accessible from your command line.

1. **Download and Install:**
   Download and install Visual Studio Code from [here](https://code.visualstudio.com/Download).
   
2. **Enable Command Line Interface:**
   - Open Visual Studio Code.
   - Access the Command Palette by pressing `CMD + SHIFT + P` (Mac) or `CTRL + SHIFT + P` (Windows/Linux).
   - Type `Shell Command: Install 'code' command in PATH` and select it.
   - Wait for a notification confirming that the `code` command has been added to your PATH.

### Building from Source
After ensuring the prerequisites are met, clone the repository and build the project with the following commands:

```bash
gh repo clone kitsunefoxx/cli-vscode
cd cli-vscode
cargo build --release
