use std::{fs::File, io::{self, Read, Write}, net::{TcpStream, Shutdown}, path::Path};

pub fn send_file(addr: &str, file_path: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(addr)?;
    let mut file = File::open(file_path)?;
    let mut buffer = [0; 1024];

    // Get the file name from the file_path
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    // Send the file name
    stream.write_all(file_name.as_bytes())?;
    stream.write_all(b"\n")?;

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        stream.write_all(&buffer[..n])?;
    }

    stream.shutdown(Shutdown::Write)?;
    println!("File successfully sent to {}", addr);
    Ok(())
}
