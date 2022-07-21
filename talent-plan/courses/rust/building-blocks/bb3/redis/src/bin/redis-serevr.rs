use std::{net::TcpListener, io::{self, Read}};



fn main() -> Result<(),io::Error>{

    let listener = TcpListener::bind("127.0.0.1:8080")?;

    let (mut tcp_stream,sock_addr) = listener.accept()?;

    dbg!(sock_addr);

    let mut buf:Vec<u8>  = Vec::with_capacity(100);

    loop {
        let len = tcp_stream.read(&mut buf)?;

        println!("{:?}",buf);
    }
    
}
