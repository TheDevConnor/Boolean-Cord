use log::error;
use std::{
    net::TcpStream,
    io
};
use super::{respond,Status};

pub fn route(_buffer: [u8; 1024], stream: &mut TcpStream) -> io::Result<()> {
    error!("UNIMPLENTED, yet");
    respond(
        format!("{}","No Message Yet"),
        Status::Ok,
        stream
    )?;
    Ok(())
}