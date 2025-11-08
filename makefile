TARGET = rrm
INSTALL_DIR= $(HOME)/.cargo/bin/rrm

build:
	cargo build --release

install:
	ln -sf $(PWD)/target/release/$(TARGET) $(INSTALL_DIR) 

clean:
	cargo clean

