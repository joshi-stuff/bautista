# Shorthand variables
CP = cp -a --no-preserve=ownership 
NODE_MODULES = node/node_modules

#
# Project targets
#
build: $(NODE_MODULES)
	cd node && yarn workspaces run build
	cd rust && cargo build

clean: $(NODE_MODULES)
	cd node && yarn workspaces run clean
	cd rust && cargo clean

format: $(NODE_MODULES) 
	cd node && yarn workspaces run format
	cd rust && cargo fmt

lint: $(NODE_MODULES)
	cd node && yarn workspaces run lint
	cd rust && cargo check

test: $(NODE_MODULES)
	cd node && yarn workspaces run test
	cd rust && cargo test

#
# Release target
#
release:
	./scripts/release

#
# Install targets
#
install:
	cd arch && rm -f *zst && makepkg -risc

#
# Internal targets
#
$(NODE_MODULES):
	cd node/bautista && yarn
