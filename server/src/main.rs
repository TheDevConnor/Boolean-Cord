use log::info;
use server::{
    ThreadPool,
    handle
};


const IP: &'static str = "0.0.0.0:443";
const THREAD_POOL_SIZE: u8 = 4;


fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind(IP)?;
    let mut pool = ThreadPool::new(THREAD_POOL_SIZE as usize);

    for stream in listener.incoming().take(THREAD_POOL_SIZE as usize) {
        handle(stream, &mut pool)?;
    };

    info!("Shutting down");
    Ok(())
}
