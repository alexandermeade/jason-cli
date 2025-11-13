# Jason CLI

A command-line tool for compiling [Jason](https://github.com/YOUR_USERNAME/jason-rs) files - a lightweight JSON templating tool.

## ✨ Features

- **Compile to JSON** - Transform `.jason` files into standard JSON
- **Pretty Printing** - Format output with `--pretty` flag
- **File Output** - Write directly to files or pipe to stdout
-  **Watch Mode** - Auto-recompile on file changes during development

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (1.70 or later)

### Installation

```bash
cargo install --git https://github.com/alexandermeade/jason-cli
```

Or from a local clone:

```bash
git clone https://github.com/alexandermeade/jason-cli
cd jason-cli
cargo install --path .
```

## 📖 Usage

### Basic Commands

```bash
# Compile to stdout
jason compile input.jason

# Compile with pretty printing
jason compile input.jason --pretty

# Compile to file
jason compile input.jason -o output.json

# Compile to file with pretty printing
jason compile input.jason -o output.json --pretty

# Watch mode (recompiles on changes)
jason watch input.jason -o output.json --pretty
```

### Help

```bash
jason --help
jason compile --help
jason watch --help
```

## Example

**Input:** `people.jason`

```jason
Person(name, age) {
    name: name,
    age: age
}

main = Person(random_name()!, random_int(67)!) * 3

out main
```

**Compile:**

```bash
jason compile people.jason --pretty -o people.json
```

**Output:** `people.json`

```json
[
  {
    "age": 41,
    "name": "Kimberly"
  },
  {
    "age": 55,
    "name": "Carol"
  },
  {
    "age": 14,
    "name": "John"
  }
]
```

## 📚 Language Specification

For full Jason language documentation, syntax, and features, see the [jason-rs documentation](https://github.com/YOUR_USERNAME/jason-rs).

## 🔧 Options

### `compile` Command

| Flag | Description |
|------|-------------|
| `-o, --output <FILE>` | Write output to file instead of stdout |
| `-p, --pretty` | Pretty print the JSON output |
| `-h, --help` | Print help information |

### `watch` Command

| Flag | Description |
|------|-------------|
| `-o, --output <FILE>` | Write output to file instead of stdout |
| `-p, --pretty` | Pretty print the JSON output |
| `-h, --help` | Print help information |

## 📦 Related Projects

- [jason-rs](https://github.com/YOUR_USERNAME/jason-rs) - The core Jason library

## 📄 License

Licensed under the **Apache License 2.0**. See [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
# jason-cli
