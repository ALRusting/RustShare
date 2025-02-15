use std::{fs::File, io::{self, Read, Write}, net::{TcpStream, Shutdown}};

pub fn send_file(addr: &str, file_path: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(addr)?;
    let mut file = File::open(file_path)?;
    let mut buffer = [0; 1024];

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        stream.write_all(&buffer[..n])?;
    }

    stream.shutdown(Shutdown::Write)?;
    println!("File inviato con successo a {}", addr);
    Ok(())
}
