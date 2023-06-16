use crate::server::lexer::Command;
use core::store;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub fn handle_command(
    command: Command,
    shards: &[Arc<Mutex<store::Store>>],
    keys: Arc<Mutex<HashSet<String>>>,
) -> Vec<u8> {
    let shard_index = get_shard_index(&command, shards.len());
    let locked_data = &mut shards[shard_index]
        .lock()
        .expect("failed to acquire lock on data");
    let mut locked_keys = keys.lock().expect("failed to acquire lock on keys");
    match command {
        Command::Get(key) => {
            if let Some(data) = locked_data.get(key.clone()) {
                data
            } else {
                locked_keys.remove(&key);
                b"Key not found".to_vec()
            }
        }
        Command::Set(key, value, expiry) => {
            locked_data.set(key.clone(), value, expiry);
            locked_keys.insert(key);
            b"Set".to_vec()
        }
        Command::Del(key) => {
            locked_data.delete(key.clone());
            locked_keys.remove(&key);
            b"Del".to_vec()
        }
        Command::Exp(key, expiry) => {
            locked_data.exp(key, expiry);
            b"Exp".to_vec()
        }
        Command::Keys(key) => {
            if let Some(value) = key {
                format!("{} \n", locked_keys.contains(&value))
                    .as_bytes()
                    .to_vec()
            } else {
                let mut data: Vec<u8> = Vec::new();
                for key in locked_keys.clone().into_iter() {
                    data.extend((key + "\n").into_bytes())
                }
                data
            }
        }
        _ => b"Invalid".to_vec(),
    }
}

fn get_shard_index(command: &Command, shard_count: usize) -> usize {
    match command {
        Command::Get(key) | Command::Del(key) => hash_key(key) % shard_count,
        Command::Set(key, _, _) => hash_key(key) % shard_count,
        Command::Exp(key, _) => hash_key(key) % shard_count,
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
