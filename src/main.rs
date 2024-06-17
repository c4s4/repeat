use anyhow::Result;
use clap::Parser;
use std::env;
use std::process::Command;
use term_size;
use colored::Colorize;

const DEFAULT_SHELL_UNIX: &str = "sh";
const DEFAULT_SHELL_WINDOWS: &str = "cmd";
const DEFAULT_TERM_WIDTH: usize = 80;

/// Repeat a command a number of times
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Number of times we repeat command
    number: u32,
    /// Command to repeat
    command: Vec<String>,
    /// Colorize output
    #[clap(short, long)]
    black: bool,
    /// Title character
    #[clap(short, long, default_value = "=")]
    character: char,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // run function
    if let Err(e) = run(args.number, args.command, args.black, &args.character.to_string()) {
        eprintln!("ERROR {:#}", e);
        std::process::exit(1);
    }
}

/// Describe function here
fn run(number: u32, command: Vec<String>, black: bool, character: &str) -> Result<()> {
    // print arguments
    println!("Run {} times '{}'", number, command.join(" "));
    // run command number times
    for i in 1..=number {
        let mut width = DEFAULT_TERM_WIDTH;
        if let Some((w, _)) = term_size::dimensions() {
            width = w;
        } else {
            eprintln!("Cannot get terminal width, using default {}", DEFAULT_TERM_WIDTH);
        }
        let message = format!("{} {} {}", str::repeat(character, 2), i, str::repeat(character, width - (i.to_string().len() + 4)));
        if black {
            println!("{}", message);
        } else {
            println!("{}", message.yellow().bold());
        }
        let status = run_command(command.clone(), None)?;
        if status != 0 {
            anyhow::bail!("Command failed with status {}", status);
        }
    }
    Ok(())
}

/// Run command
fn run_command(cmd: Vec<String>, shell: Option<String>) -> Result<i32> {
    // run command in shell
    if env::consts::OS != "windows" {
        // on unix
        let shell = shell.unwrap_or(DEFAULT_SHELL_UNIX.to_string());
        match Command::new(shell).arg("-c").arg(&cmd.join(" ")).status() {
            Ok(status) => return Ok(status.code().unwrap()),
            Err(err) => anyhow::bail!(err),
        }
    } else {
        // on windows
        let shell = shell.unwrap_or(DEFAULT_SHELL_WINDOWS.to_string());
        match Command::new(shell).arg("/c").arg(&cmd.join(" ")).status() {
            Ok(status) => return Ok(status.code().unwrap()),
            Err(err) => anyhow::bail!(err),
        }
    }
}
