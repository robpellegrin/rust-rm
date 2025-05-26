# rust-rm

An enhanced version of the Unix `rm` command written in Rust, featuring a built-in **trash bin** to safely recover accidentally deleted files.


## 🧰 Features

- 🗑️ **Trash bin compliant with FreeDesktop.org spec**:  
  Instead of permanently deleting files like `rm`, `rust-rm` moves them to the trash location defined by the [FreeDesktop.org Trash Specification](https://specifications.freedesktop.org/trash-spec/1.0/. This is the same location used by most desktop environments (e.g. GNOME, KDE), ensuring compatibility with your system's graphical trash tools.
- 🧭 **Familiar interface**:  
  Designed to mimic the traditional `rm` command-line interface for seamless replacement.
- 💬 **CLI tool**:  
  Restore, view, and empty your trash bin directly from the terminal (planned).
- ⚡ **Parallel file handling**:  
  Uses multi-threading to efficiently move large batches of files to the trash.

## 🚧 Roadmap / TODO
- Handle command-line arguments to:
  - Restore files
- Store metadata alongside deleted files to improve restoration.
- Support user-defined config files:
  - Custom trash path
  - Size limitations

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
