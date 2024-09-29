# Embedded Rust Discovery with micro:bit
This project follows the [Embedded Rust Discovery Book](https://docs.rust-embedded.org/discovery/microbit/index.html), focusing on learning and experimenting with Embedded Rust on the micro:bit platform, also added some of my own different experiments.

## Project Overview
The purpose of this repository is to document and track progress as I work through the Embedded Discovery book, using the Rust programming language to program a micro:bit device. The repository contains various examples, exercises, and personal explorations as I learn how to write embedded software using Rust.

## Branches Overview
This repository contains different branches, each corresponding to a specific chapter, example, or project code from the book or personal experiments. Each branch can be checked out and flashed onto the micro:bit.

- **`main`**: The default branch contains the starting template or most recent examples.
- **`led_roulette`**: This branch contains code that makes the LEDs on the micro:bit flash in a `roulette` pattern along the borders of the matrix.
- **`led_spiral`**: This branch contains code that lights the LEDs in a `spiral` fashion, starting from the outer border and moving towards the inner LEDs.

## Getting Started
To get started with this project, ensure you have the following:

### Prerequisites

1. **Rust**: Make sure you have the Rust toolchain installed. Install Rust using [rustup](https://rustup.rs/), and add the target device.
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add thumbv7em-none-eabihf
   ```

2. **cargo-embed**:  You will need cargo-embed for flashing the programs onto your micro. Install it via:
    ```bash
    cargo install cargo-embed
    ```

3. **GDB and OpenOCD**: You may also need GDB and OpenOCD to debug and flash (load to microcontoller) your code.
4. **microbit**: Of course, you'll need a microbit board.

## How to Flash the Code
Below command will build and flash your microbit with the current code for microbit v2:
```bash
    cargo embed --features v2 --target thumbv7em-none-eabihf
```

## Learning Resources
This project follows along with the Discovery Book for embedded Rust:

- [The Embedded Rust Discovery Book](https://docs.rust-embedded.org/discovery/microbit/index.html)
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/index.html)
- [The Rust Book](https://doc.rust-lang.org/book/)

## Acknowledgements
Thanks to the Rust Embedded Working Group for creating and maintaining the Rust Embedded Books.