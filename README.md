# TCP Server and Client in Rust

## Description

This project demonstrates a TCP server and client implementation using Rust with the Tokio async runtime. The server listens for incoming log messages from clients, buffers them, and periodically sends them to a designated destination server. The client simulates sending log messages to the server at a configurable rate.

## Features

- **TCP Server**:
  - Listens for incoming connections on a specified address and port.
  - Buffers incoming log messages in batches of 100 or every 10 seconds (whichever comes first).
  - Sends buffered messages to another destination server.

- **TCP Client**:
  - Connects to the server and sends log messages at a configurable rate.
  - Handles reconnections if the connection to the server is lost.

## Installation

To get started with this project, you'll need to have Rust and Cargo installed. Follow these steps to set up and run the project:



```markdown
## Installation and Usage


 **Build the project**:
   ```bash
   cargo build
   ```

**Run the server**:
   ```bash
   cargo run --bin server
   ```

 **Run the client**:
   ```bash
   cargo run --bin client
   ```
```


##Video:
https://github.com/sarthak20574/rust_assigment2-nebuliaq-placement-iiitd-SarthakDixit-2024
