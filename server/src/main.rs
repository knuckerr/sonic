use std::net::SocketAddr;
mod server;

fn main() {
    let server = server::Server::new();
    let addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();
    server.run(addr).unwrap();
}
