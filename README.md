# Project: JetBrains AI Evaluation

This project was designed to evaluate the capabilities of JetBrains' AI, particularly in its assistance for software
development tasks, integration, and overall user experience enhancement.

The project is a **Tetris game** built using **Rust** and **WebAssembly (WASM)**.

## Project Overview

The project utilizes Rust as the primary programming language and includes several dependencies and tools for
development. It was built to assess how effectively JetBrains' AI could be used as a tool in IntelliJ IDEA for writing,
debugging, and improving code, as well as handling developer queries in real-time.

## Dependencies

The following dependencies are used in this project:

- **[rand 0.8.5](https://crates.io/crates/rand):** Provides random number generation capabilities.
- **[wasm-bindgen 0.2.99](https://crates.io/crates/wasm-bindgen):** A library for using Rust to write and bind
  WebAssembly.
- **[web-sys 0.3.76](https://crates.io/crates/web-sys):** Low-level SC of Rust bindings for Web APIs.

## Tools & Environment

- **Rust Version:** 1.83.0
- **IntelliJ IDEA:** 2024.3.1.1 (Ultimate Edition)

## Key Features

- A basic implementation of the classic **Tetris game**.
- Written entirely in **Rust** and compiled to **WebAssembly (WASM)**.
- Runs directly in the browser by leveraging modern WASM capabilities.
- Includes essential game mechanics such as piece movement, rotation, and line clearing.

## How to Build and Use

### Prerequisites

Ensure that you have the following installed:

- Rust programming language (`rustup` recommended)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for building WebAssembly output
- A static file server or hosting environment to serve the WebAssembly output

### Build the Project

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-name>
   ```
2. Build the project using `wasm-pack`:
   ```bash
   wasm-pack build --target web
   ```

### Use the WebAssembly Output

Host the generated WebAssembly and JavaScript bindings using a static file server, such as:

```bash
python3 -m http.server
```

Open the browser at the appropriate URL to load the Tetris game.

---

## License

This project is licensed under the MIT License. For more details, refer to the `LICENSE` file in the repository.

---
