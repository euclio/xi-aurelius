PLUGIN_NAME = aurelius
PLUGIN_BIN = xi-aurelius

ifeq ($(shell uname -s), Darwin)
	XI_CONFIG_DIR ?= $(HOME)/Library/Application\ Support/XiEditor
endif

XDG_CONFIG_HOME ?= $(HOME)/.config
XI_CONFIG_DIR ?= $(XDG_CONFIG_HOME)/xi
XI_PLUGIN_DIR ?= $(XI_CONFIG_DIR)/plugins

out/$(PLUGIN_NAME): $(PLUGIN_BIN)
	mkdir -p out/$(PLUGIN_NAME)/bin
	cp target/release/$(PLUGIN_BIN) out/$(PLUGIN_NAME)/bin
	cp manifest.toml out/$(PLUGIN_NAME)/manifest.toml

.PHONY: $(PLUGIN_BIN)
$(PLUGIN_BIN):
	cargo build --release

install: manifest.toml out/$(PLUGIN_NAME)
	mkdir -p $(XI_PLUGIN_DIR)
	cp -r out/$(PLUGIN_NAME) $(XI_PLUGIN_DIR)

clean:
	rm -rf out
	cargo clean

.PHONY: clean install
