# Rust-OS

### About

An Operating System wirtten in Rust. Based on the series of tutorials [Writing an OS in Rust](https://os.phil-opp.com/). The main purpose of this project is to get a better understanding of bare metal Rust and the advantages and disadvantages of the language in OS Development.

## Build Instruction

1. Clone the Repository

    ```
    git clone https://github.com/Turtel216/Rust-OS.git
    cd Rust-OS/rust_os
    ```

2. Install the qemu-system-x86_64 Virtual Machine according to your system: [Qemu Installation Guide](https://www.qemu.org/download/#linux)

3. Install Rust nightly

    ```
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
    ```

4. Install bootimage

    ```
    cargo install bootimage
    ```

5. Install llvm-tools-preview

    ```
    rustup component add llvm-tools-preview
    ```

6. Ensure everything worked:

    ```
    cargo run
    ```

7. Run Tests

    ```
    cargo test
    ```
