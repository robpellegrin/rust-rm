# rust-rm

An enhanced version of the Unix `rm` command written in Rust, featuring a built-in **trash bin** to safely recover accidentally deleted files.


## Features

- **Parallel file handling**:
  Uses multi-threading to efficiently move large batches of files to the trash.
- **Trash bin compliant with FreeDesktop.org spec**:  
  Instead of permanently deleting files like `rm`, `rust-rm` moves them to the trash location defined by the [FreeDesktop.org Trash Specification](https://specifications.freedesktop.org/trash-spec/1.0/). This is the same location used by most desktop environments (e.g. GNOME, KDE), ensuring compatibility with your system's graphical trash tools.
- **CLI tool**:
  Restore, view, and empty your trash bin directly from the terminal.
- **Familiar interface**:
  Designed to mimic the traditional `rm` command-line interface for seamless replacement.


## Roadmap / TODO
- Handle command-line arguments to:
  - Restore files
- Support user-defined config files:
  - Custom trash path
  - Size limitations

## Building & Installation
Using the makefile:
```
make build && make install
```
This compiles rust-rm and creates a symlink at `~/.cargo/bin`

## Usage
```
Usage: rrm [OPTIONS] [FILES]...

Arguments:
  [FILES]...  list of files/directories to send to trash

Options:
  -r, --recursive   remove directories and their contents recursively
      --view-trash  list contents of trash directory
  -v, --verbose     explain what is being done
  -i                prompt before every removal
      --empty       permanently delete all files in the trash directory
  -s, --skip-trash  remove directories and their contents recursively
  -h, --help        Print help
  -V, --version     Print version
```
## License
This project is licensed under the MIT License. See the LICENSE file for details.

