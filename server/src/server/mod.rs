pub mod parser;
use core::memory_pool;
use core::store;
use core::thread_pool;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
const BUFFER_SIZE: usize = 1024;
use crate::server::parser::{parse_command, Command};

pub struct Server {
    thread_pool: thread_pool::RayonThreadPool,
}
impl Server {
    pub fn new() -> Self {
        let _thread_pool = thread_pool::RayonThreadPool::new(num_cpus::get() as u32).unwrap();
        Server {
            thread_pool: _thread_pool,
        }
    }
    pub fn run<A: ToSocketAddrs>(self, addr: A) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(addr)?;
        let memory_pool = Arc::new(Mutex::new(memory_pool::MemoryPool::new()));
        let data = Arc::new(Mutex::new(store::Store::new()));
        for stream in listener.incoming() {
            let pool = Arc::clone(&memory_pool);
            let data = Arc::clone(&data);
            self.thread_pool.spawn(move || match stream {
                Ok(stream) => {
                    if let Err(e) = handle_client(data, pool, stream) {
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
    data: Arc<Mutex<store::Store>>,
    pool: Arc<Mutex<memory_pool::MemoryPool>>,
    stream: TcpStream,
) -> Result<(), Box<dyn Error>> {
    let mut reader = stream.try_clone()?;
    let mut writer = stream.try_clone()?;

    let mut buffer = {
        let mut pool = pool.lock().expect("Failed to acquire lock on pool");
        let ptr = pool.allocate(BUFFER_SIZE);
        unsafe { std::slice::from_raw_parts_mut(ptr, BUFFER_SIZE) }
    };

    loop {
        let size = reader.read(buffer)?;
        let command = String::from_utf8_lossy(&buffer[..size]);
        let response = match parse_command(&command) {
            Command::Quit => break,
            cmd => {
                handle_command(cmd, &data)
            }
        };
        writer.write_all(&response)?;
        writer.flush()?;
        let mut pool = pool.lock().expect("Failed to acquire lock on pool");
        let ptr = buffer.as_mut_ptr();
        pool.deallocate(ptr);
        let new_ptr = pool.allocate(BUFFER_SIZE);
        buffer = unsafe { std::slice::from_raw_parts_mut(new_ptr, BUFFER_SIZE) };
    }
    let mut pool = pool.lock().expect("Failed to acquire lock on pool");
    let ptr = buffer.as_mut_ptr();
    pool.deallocate(ptr);
    Ok(())
}

fn handle_command(command: Command, data: &Arc<Mutex<store::Store>>) -> Vec<u8> {
    let mut locked_data = data.lock().expect("failed to acquire lock on data");
    match command {
        Command::Get(key) => {
            let value = locked_data.get(key);
            match value {
                Some(v) => v,
                None => b"Key not found".to_vec(),
            }
        }
        Command::Set(key, value) => {
            locked_data.set(key, value);
            b"Set".to_vec()
        }
        Command::Del(key) => {
            locked_data.delete(key);
            b"Del".to_vec()
        }
        _ => b"test".to_vec(),
    }
}
