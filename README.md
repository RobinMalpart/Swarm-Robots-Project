# Swarm-Robots-Project

Final project for Rust Module

## Description

Swarm-Robots-Project is a simulation of a swarm of robots that interact with each other and their environment. The project demonstrates various Rust programming concepts such as concurrency, parallelism, and procedural generation.

## Features

- **Random Generation**: Uses the `rand` crate for generating random seeds, obstacles, and resource placements.
- **Procedural Map Generation**: Utilizes the `noise` crate to generate maps with Perlin Noise.
- **Serialization**: Saves and loads game states using `serde` and `serde_json`.
- **Parallel Computation**: Optimizes calculations using the `rayon` crate for managing multiple robots simultaneously.
- **Asynchronous Programming**: Manages multiple tasks concurrently with the `tokio` crate.

## Dependencies

- `rand = "0.8"`
- `noise = "0.8"`
- `serde = { version = "1.0", features = ["derive"] }`
- `serde_json = "1.0"`
- `rayon = "1.7"`
- `tokio = { version = "1", features = ["full"] }`
- `crossbeam = "0.8"`
- `log = "0.4"`
- `env_logger = "0.10"`
- `ratatui = "0.24"`
- `crossterm = "0.27"`

## Usage

To run the project, use the following command:

```sh
cargo run