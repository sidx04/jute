<div align="center">
  <a href="https://github.com/sidx04/jute">
    <img src="images/logo.png" alt="Logo" width="max" height="150">
  </a>

  <p align="center">
    A minimal TUI (Terminal User Interface) application to input key-value pairs and export them as JSON!
  </p>
</div>

## ✨ Features

- Interactive TUI to enter key-value pairs.
- Outputs structured JSON to a file or standard output.
- Lightweight, no external dependencies beyond Rust standard libraries.

## 🚀 Installation

### Download Pre-built Binary

**Linux / macOS:**

```bash
curl -L https://github.com/your_username/jute/releases/download/v1.0.0/jute-tui-v1.0.0-x86_64-linux.tar.gz | tar -xz
sudo mv jute-tui /usr/local/bin/
```

**Windows:**

Download the `.zip` from [Releases](https://github.com/sidx04/jute/releases) and extract.

## 🏗️ Building from Source

Clone the repo and build:

```bash
git clone https://github.com/your_username/jute.git
cd jute
cargo build --release
```

The binary will be available at:

```
target/release/jute-tui
```

## 🛠 Usage

```bash
jute-tui > output.json
```

![Alt Text](/images/ss1.png)
