use std::{fs::File, io::{self, Read, Write}, net::{TcpListener, TcpStream}, thread};
use crate::common::PORT;

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    println!("Connessione ricevuta");
    let mut file = File::create("received_file")?;
    let mut buffer = [0; 1024];

    while let Ok(n) = stream.read(&mut buffer) {
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
    }
    println!("File ricevuto e salvato come 'received_file'");
    Ok(())
}

pub fn receive_file() -> io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT))?;
    println!("In ascolto sulla porta {}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Errore nel gestire il client: {}", e);
                    }
                });
            },
            Err(e) => eprintln!("Errore durante la connessione: {}", e),
        }
    }
    Ok(())
}
