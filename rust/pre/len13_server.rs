
use std::net::UdpSocket;

fn main() -> std::io::Result<()>{
    let socket = UdpSocket::bind("127.0.0.1:8080")?;

    println!("UDP Server online");
    let mut buf = [0;512];

    loop {
        let (len, src) = socket.recv_from(&mut buf)?;
        println!("{} {}",src, String::from_utf8_lossy(&buf[..len]));
     }
}

/*
TCP Server
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("Ricevuto {:?}",String::from_utf8_lossy(&buffer[..bytes_read]) );
    stream.write_all(b"Messaggio ricveuto\n").unwrap();
}


fn main() -> std::io::Result<()>{
    let listner = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server in ascolto");

    for stream in listner.incoming(){
        handle_client(stream?);
    }
    Ok(())
}
*/
