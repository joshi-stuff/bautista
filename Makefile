# Shorthand variables
CP = cp -a --no-preserve=ownership 
NODE_MODULES = node/node_modules

#
# Project targets
#
build: $(NODE_MODULES)
	cd node && yarn workspaces run build

clean: $(NODE_MODULES)
	cd node && yarn workspaces run clean

format: $(NODE_MODULES) 
	cd node && yarn workspaces run format

lint: $(NODE_MODULES)
	cd node && yarn workspaces run lint

test: $(NODE_MODULES)
	cd node && yarn workspaces run test

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
