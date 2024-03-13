# Actix Server

![Rust](https://img.shields.io/badge/Rust-1.56+-orange.svg)
![Actix-Web](https://img.shields.io/badge/Actix--Web-4.0-green.svg)
![serde](https://img.shields.io/badge/serde-1.0.136-blue.svg)
![dotenv](https://img.shields.io/badge/dotenv-0.15.0-yellow.svg)
![futures](https://img.shields.io/badge/futures-0.3-lightgrey.svg)
![mongodb](https://img.shields.io/badge/mongodb-2.2.0-brightgreen.svg)

This project is an Actix web server implemented in Rust. It utilizes Actix-Web for building asynchronous web applications, serde for serialization and deserialization, dotenv for environment variable management, futures for asynchronous programming, and mongodb for database connectivity.

## Requirements

- Rust 1.56+
- Cargo (Rust's package manager)

## Dependencies

- Actix-Web 4.0
- serde 1.0.136
- dotenv 0.15.0
- futures 0.3
- mongodb 2.2.0

## Usage

1. Clone the repository:

   ```
   git clone https://github.com/your_username/actix-server.git
   cd actix-server

2. Install dependencies using Cargo:

```
cargo build
cargo run
 
and for Auto Reload run :  cargo watch -x run
```

3. Replace .env with below variable

    ```MONGODB_URI=```