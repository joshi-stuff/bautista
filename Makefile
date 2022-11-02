# Shorthand variables
CP = cp -a --no-preserve=ownership 
NODE_MODULES = node/node_modules

#
# Project targets
#
build: $(NODE_MODULES)
	cd node && npm run build --workspaces
	cd rust && cargo build

clean: $(NODE_MODULES)
	cd node && npm run clean --workspaces
	cd rust && cargo clean

format: $(NODE_MODULES) 
	cd node && npm run format --workspaces
	cd rust && cargo fmt

lint: $(NODE_MODULES)
	cd node && npm run lint --workspaces
	cd rust && cargo check

test: $(NODE_MODULES)
	cd node && npm run test --workspaces
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
	cd node && npm install
