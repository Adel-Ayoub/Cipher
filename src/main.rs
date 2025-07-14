#![forbid(unsafe_code)]
mod key;
mod file;
mod crypt;
mod ui;
mod interactive;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Use the program interactively
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    /// Encrypt a file
    #[arg(short, group = "action")]
    encrypt: bool,

    /// Decrypt a file
    #[arg(short, group = "action")]
    decrypt: bool,
}

fn main() -> () {
    let args = Args::parse();
    if args.interactive {
        interactive::main();
    }
}
