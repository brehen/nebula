# Find all directories with src/main.rs
W_DIRS := $(dir $(wildcard */src/main.rs))

# Find all directory names (removing trailing slashes)
DIR_NAMES := $(patsubst %/src/,%,$(W_DIRS))

.PHONY: all build_docker build_wasm

all: build_docker build_wasm

# Pattern rule for building docker images
build_docker: $(DIR_NAMES)
	@for dir in $^; do \
		echo "#!/bin/sh" > $$dir/run.sh; \
		echo "/usr/local/bin/$$dir" >> $$dir/run.sh; \
		chmod +x $$dir/run.sh; \
		docker build --platform linux/amd64 -t nebula-function-$$dir --build-arg PROGRAM_NAME=$$dir -f Dockerfile . || true; \
		rm $$dir/run.sh; \
  done

# Pattern rule for building wasm modules
build_wasm: $(DIR_NAMES)
	@for dir in $^; do \
		cd $$dir; \
		cargo build --release --target wasm32-wasi; \
		cp target/wasm32-wasi/release/$$dir.wasm ~/projects/wasm_modules/; \
		cd ..; \
	done

# Cleanup if you need (optional)
.PHONY: clean
clean:
	# Your cleanup commands here, if any
