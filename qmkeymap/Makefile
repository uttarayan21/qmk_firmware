TARGET=thumbv6m-none-eabi
CARGO_TARGET_DIR=target

build: src/*.rs Cargo.toml build.rs
	# RUSTFLAGS="--emit obj --codegen opt-level=3 --codegen panic=abort" \
	# 	cargo build --release --target=$(TARGET)
	cargo rustc --lib --release --target=$(TARGET) \
		-- \
		--emit=obj \
		--codegen opt-level=3 --codegen panic=abort
	find --version 2>/dev/null | grep gnu && { \
		ln -sf $$(find ./target/$(TARGET)/release/deps/keymap-*.o -printf '%T+ %p\n' | sort -r | head -n 1 | awk '{print $$2}') \
			keymap.o; \
	} || { \
		ln -sf $$(find ./target/$(TARGET)/release/deps/keymap-*.o -print0 | xargs -0 stat -f '%m %N' | sort -r | head -n 1 | awk '{print $$2}') \
			keymap.o; \
	}

rustc-build:
	rustc --emit=obj \
		--target=$(TARGET) \
		--codegen opt-level=3 \
		--codegen panic=abort \
		src/keymap.rs

dump-keymap-data:
	arm-none-eabi-objdump -s -j .rodata.keymaps  keymap.o

deps:
	rustup target add $(TARGET)

clean:
	cargo clean
	-rm -f keymap.o
