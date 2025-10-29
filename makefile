TARGET = rrm
INSTALL_DIR= $(HOME)/.local/bin/rrm

build:
	cargo build --release

install:
	mkdir -p ~/.local/bin/
	ln -sf $(PWD)/target/release/$(TARGET) $(INSTALL_DIR) 

clean:
	cargo clean

