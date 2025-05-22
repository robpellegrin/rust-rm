# rust-rm

An enhanced version of the Unix `rm` command written in Rust, featuring a built-in **trash bin** to safely recover accidentally deleted files.


## 🧰 Features

- 🗑️ **Trash bin**: Instead of permanently deleting files, `rust-rm` moves them to `~/trash`, allowing easy recovery.
- 🧭 **Familiar interface**: Designed to mimic the traditional `rm` command-line interface.
- 💬 **CLI tool**: Restore, view, and empty your trash bin using command-line options (planned).
- ⚡ **Parallel file handling**: Uses multi-threading to quickly move many files at once, improving performance when deleting large batches.

## 🚧 Roadmap / TODO
- Handle command-line arguments to:
  - Restore files
- Store metadata alongside deleted files to improve restoration.
- Support user-defined config files:
  - Custom trash path
  - Size limitations
- Optionally, use the trash bin provided by your desktop environment.

## 📦 Installation

```bash
git clone https://github.com/robpellegrin/rust-rm.git
cd rust-rm
cargo build --release
```

You can then copy the binary to your path:
```bash
cp target/release/rust-rm ~/.local/bin/
```

## 🤝 Collaborations

Contributions, feedback, and feature ideas are always welcome!

- Check out the [Issues](https://github.com/robpellegrin/rust-rm/issues) page for bugs or features to work on.
- Open a [Discussion](https://github.com/robpellegrin/rust-rm/discussions) if you have suggestions or questions.

## Author
Robert Pellegrin

## License
This project is licensed under the MIT License. See the LICENSE file for details.

Made with ❤️ in Rust
