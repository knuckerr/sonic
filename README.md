# sonic
This is a toy Redis implementation in Rust, designed for learning purposes. It provides a limited set of commands and includes features such as threading, memory shards, data compression using gzip, and decompression upon receiving data.

## Commands

The following commands are supported by this toy Redis implementation:

- GET: Retrieves the value associated with a given key.
- SET: Sets the value for a specified key.
- DEL: Deletes a key and its associated value.
- EXP: Expires a key after a specified period.

Feel free to explore and experiment with these commands to understand the basic functionality of Redis.

## Server-side Code

The server-side code of this toy Redis implementation is written in Rust. It leverages threading and memory shards to enhance performance and handle multiple connections concurrently. Threading allows for parallel execution of tasks, improving responsiveness and scalability.

In addition, the data stored in this implementation is compressed using gzip. Gzip is a widely-used compression algorithm that reduces the size of data, thereby optimizing storage and network transfer. The compressed data is automatically decompressed upon receiving, ensuring seamless communication with the client.

## Getting Started

To get started with this toy Redis implementation, follow these steps:

1. Clone the repository: `git clone https://github.com/knuckerr/sonic.git`.
2. Install Rust and Cargo if you haven't already: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
3. Navigate to the project directory: `cd sonic/server`.
4. Build the project: `cargo build`.
5. Run the server: `cargo run`.

Once the server is running, you can connect to it using the Python client or by sending commands directly via a TCP connection. Refer to the client documentation for further instructions on connecting and interacting with the server.

## Contributing

Contributions to this toy Redis implementation are welcome! If you find any issues or have ideas for improvements, please open an issue or submit a pull request on the GitHub repository.

## License

This project is licensed under the MIT License. Feel free to use and modify it according to your needs. Refer to the `LICENSE` file for more information.

## Disclaimer

Please note that this implementation is intended for educational purposes and should not be used in production environments. It lacks many of the advanced features, optimizations, and robustness found in mature Redis implementations.
