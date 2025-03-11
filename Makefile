# List of directories
BIN := target

all: 
	cargo build --release

debug: 
	cargo build 

test:
	cargo build --release
	cargo test

clean:
	rm -rf $(BIN)
	cargo clean
