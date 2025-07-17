# git404 — A Lightweight Git Implementation in Rust 🚀

`git404` is a custom-built version control system inspired by Git, written entirely in Rust. It aims to provide a deeper understanding of how Git works under the hood by replicating core functionalities like initializing repositories, committing changes, and managing objects.

> ⚙️ Built with performance, learning, and extensibility in mind.

---

## 📦 Features

- 🔧 `init` — Initialize a new repository
- 📂 Object storage using Git-like `.git/objects`
- 🧠 SHA-1 based content-addressed storage
- 🗂️ Simple file snapshotting
- ⚙️ Expandable command architecture using Clap

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Git (for comparing behaviors)

### Installation

Clone the repository and build the binary:

```bash
git clone https://github.com/yourusername/git404.git
cd git404
cargo build --release
```

---

## 🛠️ Usage

To initialize a new repository using `git404`:

```bash
./target/release/git404 init
```

This will create a `.git404` directory in your current folder, similar to how Git sets up `.git`.

---

## 📁 Project Structure

```text
.
├── src/
│   ├── main.rs          # CLI entry point
│   └── utils.rs         # Core logic for init and future Git commands
├── Cargo.toml           # Rust package config
└── README.md
```

---

## 🧠 Motivation

The goal of `git404` is not to replace Git, but to **learn how Git actually works internally** by rebuilding it from scratch using Rust. This is a great project for:

- Learning low-level file and content manipulation
- Understanding Git’s object model (blobs, trees, commits)
- Exploring CLI design with `clap` and idiomatic Rust

---

## 📅 Roadmap

- [x] `init` command
- [ ] `hash-object`
- [ ] `cat-file`
- [ ] `add`
- [ ] `commit`
- [ ] `log`
- [ ] Branching and checkout

---

## 🙌 Contributing

Feel free to fork this project, suggest improvements, or open issues. Contributions are welcome!

---

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## ✨ Acknowledgements

- Inspired by [Git from the Bottom Up](https://jwiegley.github.io/git-from-the-bottom-up/)
- Built using [Rust](https://www.rust-lang.org/) and [Clap](https://docs.rs/clap/)

---

### 🚧 Note

This is a work-in-progress. Expect bugs, missing features, and frequent changes.

---
