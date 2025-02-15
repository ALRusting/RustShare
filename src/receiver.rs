use std::{fs::{self, File}, io::{self, BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}, thread};
use crate::common::PORT;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("Connection received");

    // Read the file name
    let mut reader = BufReader::new(&mut stream);
    let mut file_name = String::new();
    reader.read_line(&mut file_name)?;
    let file_name = file_name.trim();

    // Create the "received" directory if it doesn't exist
    fs::create_dir_all("received")?;

    let file_path = format!("received/{}", file_name);
    let mut file = File::create(file_path)?;
    let mut buffer = [0; 1024];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
    }
    println!("File received and saved as 'received/{}'", file_name);
    Ok(())
}

pub fn receive_file() -> io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT))?;
    println!("Listening on port {}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            },
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }
    Ok(())
}
