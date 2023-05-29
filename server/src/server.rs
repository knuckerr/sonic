use core::memory_pool;
use core::store;
use core::thread_pool;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};

const BUFFER_SIZE: usize = 1024;

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
                    handle_client(data, pool, stream);
                }
                Err(e) => println!("Connection failed: {}", e),
            })
        }
        Ok(())
    }
}

fn handle_client(
    data: Arc<Mutex<store::Store<'_>>>,
    pool: Arc<Mutex<memory_pool::MemoryPool>>,
    stream: TcpStream,
) {
    let mut reader =
        snap::read::FrameDecoder::new(stream.try_clone().expect("Failed to clone stream"));
    let mut writer =
        snap::write::FrameEncoder::new(stream.try_clone().expect("Failed to clone stream"));

    let mut buffer = {
        let mut pool = pool.lock().expect("Failed to acquire lock on pool");
        let ptr = pool.allocate(BUFFER_SIZE);
        unsafe { std::slice::from_raw_parts_mut(ptr, BUFFER_SIZE) }
    };

    loop {
        let result = reader.read(buffer);
        match result {
            Ok(0) | Err(_) => break,
            Ok(size) => {
                let command = String::from_utf8_lossy(&buffer[..size]);
                let response = match command.trim() {
                    "quit" => break,
                    cmd => {
                        let test = "OK";
                        test.as_bytes()
                    }
                };
                writer
                    .write_all(response)
                    .expect("Failed to write response");
                writer.flush().expect("Failed to flush writer");
            }
        }
        let mut pool = pool.lock().expect("Failed to acquire lock on pool");
        let ptr = buffer.as_mut_ptr();
        pool.deallocate(ptr);
        let new_ptr = pool.allocate(BUFFER_SIZE);
        buffer = unsafe { std::slice::from_raw_parts_mut(new_ptr, BUFFER_SIZE) };
    }
    let mut pool = pool.lock().expect("Failed to acquire lock on pool");
    let ptr = buffer.as_mut_ptr();
    pool.deallocate(ptr);
}
