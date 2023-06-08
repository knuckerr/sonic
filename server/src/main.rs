use std::net::SocketAddr;
mod server;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value_t = ("127.0.0.1".to_string()))]
    ip: String,
    #[arg(short, long, default_value_t = 8000)]
    port: u16,
    #[arg(short, long, default_value_t = 1024)]
    buffer_size: usize,
    #[arg(short, long, default_value_t = 4)]
    shards: usize,
    #[arg(short, long, default_value_t = 6)]
    threads: u32,
}

fn main() {
    let cli = Cli::parse();
    let address = format!("{}:{}", cli.ip, cli.port);
    let server = server::Server::new(cli.buffer_size, cli.shards, cli.threads);
    let addr: SocketAddr = address.parse().unwrap();
    server.run(addr).unwrap();
}
