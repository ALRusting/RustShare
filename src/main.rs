use std::env;
use std::io;
mod sender;
mod receiver;
mod common;

fn print_usage() {
    println!("Usage:");
    println!("  send <address> <file_path> - Send a file to the specified address");
    println!("  receive - Receive files on the specified port");
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 4 && args[1] == "send" {
        sender::send_file(&args[2], &args[3])?;
    } else {
        receiver::receive_file()?;
    }
    Ok(())
}
