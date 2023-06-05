use crate::server::lexer::Command;
use core::store;
use std::sync::{Arc, Mutex};

pub fn handle_command(command: Command, shards: &[Arc<Mutex<store::Store>>]) -> Vec<u8> {
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
        Command::Exp(key, expiry) => {
            locked_data.exp(key, expiry);
            b"Exp".to_vec()
        }
        _ => b"Invalid".to_vec(),
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
