use std::{
    io::{self, prelude::*},
    net::TcpStream
};
use crate::routes;


pub fn handle(stream: Result<TcpStream, io::Error>, pool: &mut crate::workers::ThreadPool) -> io::Result<()> {
    let stream = stream.unwrap();

    pool.execute(|| {
        handle_connection(stream)
    });

    Ok(())
}


fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(
        b"POST /signup HTTP/1.1\r\n"
    ) {
        routes::signup::route(buffer, &mut stream)
    } else if buffer.starts_with(
        b"POST /signup HTTP/1.1\r\n"
    ) {
        routes::login::route(buffer, &mut stream)
    } else {
        routes::void::route(buffer, &mut stream)
    }
}
