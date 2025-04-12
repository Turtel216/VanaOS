# VanaOS

> A microkernel operating system written in Rust, built for learning and experimentation.

---

## 🚀 About

**VanaOS** is a hobby operating system developed in Rust, inspired by the excellent [Writing an OS in Rust](https://os.phil-opp.com/) tutorial series by Philipp Oppermann.

The primary goal of this project is to gain a deeper understanding of:
- **Low-level systems programming**
- **Bare-metal development**
- **Operating system concepts**
- The **advantages and limitations of using Rust** for OS development

This project is built from scratch with no standard library, targeting `x86_64` architecture.

---

## 🛠️ Features

- ✅ Bare-metal kernel written in Rust
- ✅ Custom minimal bootloader using [`bootimage`](https://github.com/rust-osdev/bootimage)
- ✅ Runs on `qemu-system-x86_64`
- ✅ Paging, memory management
- 🚧 Task scheduling, and system calls (in progress)
- 🧪 Unit and integration tests for kernel components

---

## 🧰 Prerequisites

Before building VanaOS, make sure the following are installed on your system:

- [Rust (nightly)](https://rustup.rs/)
- [QEMU (x86_64)](https://www.qemu.org/download/)
- `bootimage` and `llvm-tools-preview` components

---

## 🧱 Build Instructions

1. **Clone the Repository**
   ```bash
   git clone https://github.com/Turtel216/VanaOS.git
   cd VanaOS
   ```

2. **Install QEMU**
   Follow the instructions for your platform: [QEMU Installation Guide](https://www.qemu.org/download/)

3. **Install Rust nightly toolchain**
   ```bash
   rustup install nightly
   rustup override set nightly
   rustup component add rust-src --toolchain nightly
   ```

4. **Install required components**
   ```bash
   cargo install bootimage
   rustup component add llvm-tools-preview
   ```

5. **Build and run the OS**
   ```bash
   cargo run
   ```

6. **Run tests**
   ```bash
   cargo test
   ```

---

## 🔍 Project Structure

```
VanaOS/
├── src/                # Kernel source code
├── Cargo.toml          # Project manifest
└── .cargo/config.toml  # Build target configuration
```

---

## 📚 Resources & Inspirations

- [Writing an OS in Rust](https://os.phil-opp.com/)
- [Rust OSDev](https://osdev.org/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [QEMU Documentation](https://wiki.qemu.org/Main_Page)

---

## 🤝 Contributing

This is a personal learning project, but contributions, suggestions, or ideas are always welcome! Feel free to open issues or submit pull requests.

---

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
