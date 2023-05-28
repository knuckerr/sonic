use core::memory_pool;
use core::store;

fn main() {
    let mut pool = memory_pool::MemoryPool::new();
    let mut store = store::Store::new();
    println!("Hello, world!");
}
