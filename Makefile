.PHONY= package

package: target/release/ports
	./target/release/ports package ports.spec

target/release/ports:
	cargo build


