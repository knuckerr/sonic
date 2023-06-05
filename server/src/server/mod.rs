pub mod lexer;
pub mod parser;

use core::store;
use core::thread_pool;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};

use crate::server::lexer::{lexer_command, Command};
use crate::server::parser::handle_command;

pub struct Server {
    thread_pool: thread_pool::RayonThreadPool,
    buffer_size: usize,
    shard_count: usize,
}
impl Server {
    pub fn new() -> Self {
        let _thread_pool = thread_pool::RayonThreadPool::new(num_cpus::get() as u32).unwrap();
        Server {
            thread_pool: _thread_pool,
            buffer_size: 90000,
            shard_count: 4,
        }
    }
    pub fn create_shards(&self) -> Vec<Arc<Mutex<store::Store>>> {
        let mut shards = Vec::with_capacity(self.shard_count);
        for _ in 0..self.shard_count {
            shards.push(Arc::new(Mutex::new(store::Store::new())));
        }
        shards
    }
    pub fn run<A: ToSocketAddrs>(self, addr: A) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;
        let shards = self.create_shards();
        for stream in listener.incoming() {
            let shards = shards.clone();
            self.thread_pool.spawn(move || match stream {
                Ok(stream) => {
                    if let Err(e) = handle_client(&shards, self.buffer_size, stream) {
                        println!("{}", e)
                    }
                }
                Err(e) => println!("Connection failed: {}", e),
            })
        }
        Ok(())
    }
}

fn handle_client(
    shards: &[Arc<Mutex<store::Store>>],
    buffer_size: usize,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0u8; buffer_size];

    let mut reader = stream.try_clone()?;
    let mut writer = stream.try_clone()?;

    loop {
        let size = reader.read(&mut buffer)?;
        let mut command = String::new();

        // Decompress the received command
        let mut decoder = GzDecoder::new(&buffer[..size]);
        decoder.read_to_string(&mut command)?;

        let response = match lexer_command(&command) {
            Command::Quit => break,
            cmd => handle_command(cmd, shards),
        };

        // Compress the response
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&response)?;
        let compressed_data = encoder.finish()?;

        writer.write_all(&compressed_data)?;
        writer.flush()?;
    }

    Ok(())
}
