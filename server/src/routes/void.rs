use std::{
    io::{self,BufRead},
    net::TcpStream
};
use super::{respond,Status};


pub fn route(buffer: [u8; 1024], stream: &mut TcpStream) -> io::Result<()> {
    let http_request: Vec<_> = buffer
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    respond(
        format!("{:?}",http_request),
        Status::NotFound,
        stream
    )?;
    Ok(())
}