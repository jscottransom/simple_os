# 🧵 simple_os

A minimalist operating system crafted from scratch in Rust, inspired by Phil Oppermann's renowned blog series and updated to align with modern compiler constructs.

## 🛠️ Overview

**simple_os** takes a step-by-step approach to building an OS kernel in Rust, targeting the x86_64 architecture. Driven by low-level systems programming, this project highlights:

- Bootloading and memory management
- Interrupt handling and system calls
- Kernel development without the standard library

## 🚀 Getting Started

### Prerequisites

- **Rust nightly toolchain**: Install via [rustup](https://rustup.rs/)
- **QEMU**: For emulation
- **x86_64-unknown-none** target: Add using `rustup target add x86_64-unknown-none`

### Building and Running

1. Clone the repository:
   ```bash
   git clone https://github.com/jscottransom/simple_os.git
   cd simple_os
2. Build:
   ```bash
   cargo build --release
3. Run:
   ```bash
   cargo run
