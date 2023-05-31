import socket
import snappy

BUFFER_SIZE = 1024

def main():
    # Connect to the server
    host = "127.0.0.1"
    port = 8000
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect((host, port))

    while True:
        # Read user input from the command line
        command = input("Enter a command (get/set/del/quit): ")

        # Compress the command using Snappy
        compressed_command = command.encode()

        # Send the compressed command to the server
        client_socket.sendall(compressed_command)

        # Receive and process the response from the server
        response = client_socket.recv(BUFFER_SIZE)
        # Decompress the response using Snappy
        print("Server response:", response.decode())

        # Check if the client should quit
        if command == "quit":
            break

    # Close the socket connection
    client_socket.close()

if __name__ == "__main__":
    main()
