# ──── Global Variables ────────────────────────────────────────────────────
SHELL := /usr/bin/zsh
CURRENT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))
BUILD_OUTPUT_FILE := $(CURRENT_DIR)target/debug/forge
BUILD_DST_FILE := /usr/bin/forge

FORGE_ETC_DIR := /etc/forge
FORGE_CONFIG_SRC := $(CURRENT_DIR)forge.conf
FORGE_CONFIG_DST := $(FORGE_ETC_DIR)/forge.conf

# ──── Colors and Helpers ──────────────────────────────────────────────────
RED     := \033[0;31m
GREEN   := \033[0;32m
YELLOW  := \033[0;33m
BLUE    := \033[0;34m
CYAN	:= \033[0;36m
RESET   := \033[0m
define start_step_message
	@echo -e "\n$(CYAN)[*] $(1) [*]$(RESET)"
endef
define error_message
	@echo "$(RED)ERROR$(RESET): $(1)"
endef
define successful
	@echo -e "\t - $(GREEN)*Successful*$(RESET)\n"
endef

# ──── Entrypoint ──────────────────────────────────────────────────────────
all: prep_dirs build_forge

prep_dirs:
	@sudo mkdir -p $(FORGE_ETC_DIR)
	@sudo cp $(FORGE_CONFIG_SRC) $(FORGE_CONFIG_DST)
	@sudo rm -rf $(BUILD_DST_FILE)

build_forge:					## Builds the Forge binary and moves it to destination
	$(call start_step_message,"Building '$(BUILD_DST_FILE)'")
	@cargo build
	@sudo mv -f $(BUILD_OUTPUT_FILE) $(BUILD_DST_FILE)
	@rehash
	$(call successful)

clean:							## Cleans all output artifacts
	$(call start_step_message,"Cleaning Build Artifacts")
	@rm -rf $(CURRENT_DIR)/target $(BUILD_DST_FILE)
	$(call successful)

help:							## Displays available make targets
	@egrep -h '\s##\s' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "$(BLUE)  %-30s$(RESET) %s\n", $$1, $$2}'