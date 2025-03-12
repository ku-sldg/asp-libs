BIN := target

# list of targets to not build by default
DEFAULT_EXCLUDES ?= sig_tpm sig_tpm_appr

default:
	cargo build --release --workspace $(foreach exclude,$(DEFAULT_EXCLUDES),--exclude $(exclude))

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
