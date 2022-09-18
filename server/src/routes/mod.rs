use std::{
    net::TcpStream,
    fmt::{
        Formatter,
        Display,
        self
    },
    io::{
        Write,
        self
    }
};

pub mod signup;
pub mod login;
pub mod void;

pub(crate) enum Status {
    Ok,
    NotFound
}
impl Status {
    fn to_str(&self) -> String {
        match &self {
            Status::Ok => String::from("HTTP/1.1 200 OK"),
            Status::NotFound => String::from("HTTP/1.1 404 NOT FOUND")
        }
    }
}
impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.to_str())
    }
}


pub(crate) fn respond(contents: String, status: Status, stream: &mut TcpStream) -> io::Result<()> {
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}