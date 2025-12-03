format: 
	cargo fmt --all
build: 
	cargo build-sbf
test: 
	SBF_OUT_DIR=$(PWD)/target/deploy cargo test --package test-program