# git404 — A Lightweight Git Implementation in Rust 🚀

`git404` is a custom-built version control system inspired by Git, written entirely in Rust. It aims to provide a deeper understanding of how Git works under the hood by replicating core functionalities like initializing repositories, committing changes, and managing objects — without relying on Git’s plumbing commands.

> ⚙️ Built with performance, learning, and extensibility in mind.

---

## 📦 Features

- 🔧 `init` — Initialize a new repository (`.git404/`)
- 📄 `write-blob` — Store file content as SHA-1 blob
- 📖 `read-blob` — Read and decode stored blob content
- 🌲 `read-tree` — Reconstruct tree object to visualize project structure
- 📝 `write-tree` — (Coming Soon) Save file structure as tree object
- 🧱 `write-commit` — (Planned) Create commit objects linking trees and parents
- 🔐 SHA-1 based content-addressed storage
- 📂 Git-like object storage in `.git404/objects`
- 🛠️ Modular architecture using `clap`

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Git (optional — for comparing behaviors)

### Installation

Clone the repository and build the binary:

```bash
git clone https://github.com/vishaljoshi789/git404.git
cd git404
cargo build --release
```

---

## 🛠️ Usage

To initialize a new repository using `git404`:

```bash
./target/release/git404 init
```

To write a blob from a file:

```bash
./target/release/git404 write-blob <file_path>
```

To read a blob by its hash:

```bash
./target/release/git404 read-blob <hash>
```

To read a tree object:

```bash
./target/release/git404 read-tree <tree_hash>
```

> 📌 More commands like `write-tree` and `write-commit` will be available soon!

---

## 📁 Project Structure

```text
.
├── src/
│   ├── main.rs          # CLI entry point
│   ├── commands/        # Modular command handlers
│   │   ├── init.rs
│   │   ├── blob.rs
│   │   ├── tree.rs
│   └── utils.rs         # Shared logic (SHA-1, file IO)
├── Cargo.toml           # Rust package config
└── README.md
```

---

## 🧠 Motivation

The goal of `git404` is to **learn how Git actually works internally** by rebuilding its basic features from scratch using Rust — but instead of calling Git’s own plumbing commands, everything is implemented manually to replicate the behavior.

This project attempts to teach how Git stores and links objects internally — from hashing blobs to creating commits and managing trees — all while writing your own logic.

---

## 🙌 Contributing

Feel free to fork this project, suggest improvements, or open issues. Contributions are welcome!

---

## ✨ Acknowledgements

- Inspired by [CodeCrafters](https://codecrafters.io/)
- Based on Git’s internal data model and low-level architecture

---

### 🚧 Note

This is a work-in-progress. Expect bugs, missing features, and frequent changes as more commands are implemented.
