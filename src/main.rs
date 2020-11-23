mod net;
mod objects;
mod util;

use async_std::task;
use net::udp;
use std::io;

async fn async_main() -> io::Result<()> {
    let mut server = udp::Server::new("127.0.0.1:22023").await?;
    Ok(server.listen_blocking().await)
}

fn main() -> std::io::Result<()> {
    task::block_on(async_main())
}
