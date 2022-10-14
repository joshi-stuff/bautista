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
aur:
	cd arch && rm *zst && makepkg -risc

install: 
	mkdir -p "/usr/bin"
	$(CP) workspace/target/release/my-sway "/usr/bin/my-sway"

	mkdir -p "/usr/lib/my-sway"
	$(CP) -R src/lib/* "/usr/lib/my-sway"

	mkdir -p "/usr/share/my-sway"
	$(CP) -R src/share/* "/usr/share/my-sway"

	mkdir -p "/etc/greetd"
	$(CP) -R src/etc/greetd/environments "/etc/greetd"
	$(CP) -R src/etc/greetd/gtkgreet-sway.conf "/etc/greetd"

	mkdir -p "/etc/sudoers.d"
	chmod 750 "/etc/sudoers.d"
	$(CP) -R src/etc/sudoers.d/50_my-sway "/etc/sudoers.d"

	mkdir -p "/etc/systemd/logind.conf.d"
	$(CP) -R src/etc/systemd/logind.conf.d/50-my-sway.conf "/etc/systemd/logind.conf.d"

	mkdir -p "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway.slice "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway.target "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway-clipboard-manager.service "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway-idle-manager.service "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway-keystore.service "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway-mtp-automounter.service "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway-notification-server.service "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway-polkit-agent.service "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway-redshift.service "/usr/lib/systemd/user"
	$(CP) -R src/etc/systemd/user/mysway-udisks2-automounter.service "/usr/lib/systemd/user"

uninstall:
	rm -f "/usr/bin/my-sway"

	rm -rf "/usr/lib/my-sway"

	rm -rf "/usr/share/my-sway"

	rm -f "/etc/greetd/environments"
	rm -f "/etc/greetd/gtkgreet-sway.conf"

	rm -f "/etc/sudoers.d/50_my-sway"

	rm -f "/etc/systemd/logind.conf.d/50-my-sway.conf"

	rm -f "/usr/lib/systemd/user/mysway.slice"
	rm -f "/usr/lib/systemd/user/mysway.target"
	rm -f "/usr/lib/systemd/user/mysway-clipboard-manager.service"
	rm -f "/usr/lib/systemd/user/mysway-idle-manager.service"
	rm -f "/usr/lib/systemd/user/mysway-keystore.service"
	rm -f "/usr/lib/systemd/user/mysway-mtp-automounter.service"
	rm -f "/usr/lib/systemd/user/mysway-notification-server.service"
	rm -f "/usr/lib/systemd/user/mysway-polkit-agent.service"
	rm -f "/usr/lib/systemd/user/mysway-redshift.service"
	rm -f "/usr/lib/systemd/user/mysway-udisks2-automounter.service"

#
# Internal targets
#
$(NODE_MODULES):
	cd node/bautista && yarn
