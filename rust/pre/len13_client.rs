
use std::net::UdpSocket;

fn main()-> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.send_to(b"ciao udp server", "127.0.0.1:8080")?;
    Ok(())
}
/*

TCP Client

use std::net::TcpStream;
use std::io::{self, Write, Read};


fn main() -> io::Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write_all(b"ciao server")?;

    let mut buffer = [0; 512];
    let n = stream.read(&mut buffer)?;
    println!("Risposta {:?}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}
*/
