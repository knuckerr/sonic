pub mod parser;
use core::memory_pool;
use core::store;
use core::thread_pool;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use crate::server::parser::{parse_command, Command};

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
            buffer_size: 1024,
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
        let command = String::from_utf8_lossy(&buffer[..size]);

        let response = match parse_command(&command) {
            Command::Quit => break,
            cmd => handle_command(cmd, shards),
        };

        writer.write_all(&response)?;
        writer.flush()?;
    }

    Ok(())
}

fn handle_command(command: Command, shards: &[Arc<Mutex<store::Store>>]) -> Vec<u8> {
    let shard_index = get_shard_index(&command, shards.len());
    let locked_data = &mut shards[shard_index]
        .lock()
        .expect("failed to acquire lock on data");
    match command {
        Command::Get(key) => locked_data
            .get(key)
            .map_or_else(|| b"Key not found".to_vec(), |v| v),
        Command::Set(key, value, expiry) => {
            locked_data.set(key, value, expiry);
            b"Set".to_vec()
        }
        Command::Del(key) => {
            locked_data.delete(key);
            b"Del".to_vec()
        }
        _ => b"test".to_vec(),
    }
}

fn get_shard_index(command: &Command, shard_count: usize) -> usize {
    match command {
        Command::Get(key) | Command::Del(key) => hash_key(key) % shard_count,
        Command::Set(key, _, _) => hash_key(key) % shard_count,
        _ => 0,
    }
}

fn hash_key(key: &str) -> usize {
    let mut hash = 0;
    for byte in key.bytes() {
        hash = (hash + byte as usize) % usize::MAX
    }
    hash
}
