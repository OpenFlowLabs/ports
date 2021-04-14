.PHONY: package clean all

all: clean package

package: target/release/ports
	./target/release/ports package ports.spec

target/release/ports:
	cd ports; cargo build --release

clean:
	cargo clean