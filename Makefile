# Makefile for Calimero x ICP PoW Mining Project

# Configuration
DFX_VERSION=0.24.3
CARGO_VERSION=1.81.0
CANDID_EXTRACTOR_VERSION=0.1.5
PNPM_VERSION=9.6.0
CALIMERO_APP_WASM_PATH=./src/backend/res/hello_app.wasm
CALIMERO_APP_BUILD_PATH=./src/backend/

# Control flags for each package
# Set DISABLE to 1 to skip the check
DFX_DISABLE=0
CARGO_DISABLE=0
CANDID_EXTRACTOR_DISABLE=0
PNPM_DISABLE=0

# Set to 1 for exact version matching, 0 for minimum version
DFX_EXACT_VERSION=1
CARGO_EXACT_VERSION=0
CANDID_EXTRACTOR_EXACT_VERSION=1
PNPM_EXACT_VERSION=0

NODE_NAME=default
SERVER_PORT=2428
SWARM_PORT=2528

# Complete setup process
all: check-prerequisites setup-icp-devnet init-calimero-nodes
	@echo "✓ Complete setup process finished successfully!"
	@echo "ICP devnet is running"
	@echo "Calimero nodes are initialized and running"
	@echo "Configuration saved in .env file"

# Check if cargo is installed and has correct version
check-cargo:
	@if [ $(CARGO_DISABLE) -eq 1 ]; then \
		echo "Cargo check disabled, skipping..."; \
		exit 0; \
	fi
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "cargo is not installed. Please install Rust and Cargo from https://rustup.rs/"; \
		exit 1; \
	fi
	@CURRENT_VERSION=$$(cargo --version | awk '{print $$2}'); \
	echo "Found cargo version: $$CURRENT_VERSION"; \
	if [ $(CARGO_EXACT_VERSION) -eq 1 ]; then \
		if ! echo "$$CURRENT_VERSION" | grep -q "^$(CARGO_VERSION)"; then \
			echo "Wrong cargo version. Current: $$CURRENT_VERSION, Required: $(CARGO_VERSION)"; \
			exit 1; \
		fi; \
		echo "✓ Cargo version matches exactly: $$CURRENT_VERSION"; \
	else \
		if ! echo "$$CURRENT_VERSION" | awk -v ver="$(CARGO_VERSION)" '{ if ($$1 < ver) exit 1; }'; then \
			echo "Cargo version too old. Current: $$CURRENT_VERSION, Minimum required: $(CARGO_VERSION)"; \
			exit 1; \
		fi; \
		echo "✓ Cargo version ($$CURRENT_VERSION) meets minimum requirement: $(CARGO_VERSION)"; \
	fi

# Check if candid-extractor is installed and has correct version
check-candid-extractor:
	@if [ $(CANDID_EXTRACTOR_DISABLE) -eq 1 ]; then \
		echo "Candid-extractor check disabled, skipping..."; \
		exit 0; \
	fi
	@if ! command -v candid-extractor >/dev/null 2>&1; then \
		echo "candid-extractor is not installed. Installing..."; \
		cargo install candid-extractor --version $(CANDID_EXTRACTOR_VERSION); \
	fi
	@CURRENT_VERSION=$$(candid-extractor --version); \
	echo "Found candid-extractor version: $$CURRENT_VERSION"; \
	if [ $(CANDID_EXTRACTOR_EXACT_VERSION) -eq 1 ]; then \
		if ! echo "$$CURRENT_VERSION" | grep -q $(CANDID_EXTRACTOR_VERSION); then \
			echo "Wrong candid-extractor version. Current: $$CURRENT_VERSION, Required: $(CANDID_EXTRACTOR_VERSION)"; \
			cargo install candid-extractor --version $(CANDID_EXTRACTOR_VERSION); \
		fi; \
		echo "✓ Candid-extractor version matches exactly: $$CURRENT_VERSION"; \
	fi

# Check if pnpm is installed and has correct version
check-pnpm:
	@if [ $(PNPM_DISABLE) -eq 1 ]; then \
		echo "PNPM check disabled, skipping..."; \
		exit 0; \
	fi
	@if ! command -v pnpm >/dev/null 2>&1; then \
		echo "pnpm is not installed. Installing..."; \
		npm install -g pnpm@$(PNPM_VERSION); \
	fi
	@CURRENT_VERSION=$$(pnpm --version); \
	echo "Found pnpm version: $$CURRENT_VERSION"; \
	if [ $(PNPM_EXACT_VERSION) -eq 1 ]; then \
		if ! echo "$$CURRENT_VERSION" | grep -q $(PNPM_VERSION); then \
			echo "Wrong pnpm version. Current: $$CURRENT_VERSION, Required: $(PNPM_VERSION)"; \
			npm install -g pnpm@$(PNPM_VERSION); \
		fi; \
		echo "✓ PNPM version matches exactly: $$CURRENT_VERSION"; \
	else \
		if ! echo "$$CURRENT_VERSION" | awk -v ver="$(PNPM_VERSION)" '{ if ($$1 < ver) exit 1; }'; then \
			echo "pnpm version too old. Current: $$CURRENT_VERSION, Minimum required: $(PNPM_VERSION)"; \
			npm install -g pnpm@$(PNPM_VERSION); \
		fi; \
		echo "✓ PNPM version ($$CURRENT_VERSION) meets minimum requirement: $(PNPM_VERSION)"; \
	fi

# Check if dfx is installed and has correct version
check-dfx:
	@if [ $(DFX_DISABLE) -eq 1 ]; then \
		echo "DFX check disabled, skipping..."; \
		exit 0; \
	fi
	@if ! command -v dfx >/dev/null 2>&1; then \
		echo "dfx is not installed. Installing..."; \
		sh -ci "$$(curl -fsSL https://internetcomputer.org/install.sh)"; \
	fi
	@CURRENT_VERSION=$$(dfx --version); \
	echo "Found dfx version: $$CURRENT_VERSION"; \
	if [ $(DFX_EXACT_VERSION) -eq 1 ]; then \
		if ! echo "$$CURRENT_VERSION" | grep -q $(DFX_VERSION); then \
			echo "Wrong dfx version. Current: $$CURRENT_VERSION, Required: $(DFX_VERSION)"; \
			dfxvm default $(DFX_VERSION); \
		fi; \
		echo "✓ DFX version matches exactly: $$CURRENT_VERSION"; \
	else \
		if ! echo "$$CURRENT_VERSION" | awk -v ver="$(DFX_VERSION)" '{ if ($$1 < ver) exit 1; }'; then \
			echo "dfx version too old. Current: $$CURRENT_VERSION, Minimum required: $(DFX_VERSION)"; \
			dfxvm default $(DFX_VERSION); \
		fi; \
		echo "✓ DFX version ($$CURRENT_VERSION) meets minimum requirement: $(DFX_VERSION)"; \
	fi

# Check all prerequisites
check-prerequisites: check-dfx check-cargo check-candid-extractor check-pnpm
	@echo "✓ All prerequisites checked successfully"

# Build node application WASM
build-node-app-wasm:
	@echo "Building node application WASM..." && \
	echo "Printing working directory: $(shell pwd)" && \
	cd $(CALIMERO_APP_BUILD_PATH) && \
	chmod +x ./build.sh && \
	./build.sh && \
	echo "✓ Node application WASM built successfully"

# Setup ICP devnet environment
setup-icp-devnet: check-prerequisites build-node-app-wasm
	@echo "Starting ICP devnet deployment..."
	@./tools/deploy_devnet.sh
	@echo "✓ ICP devnet environment setup completed"

# Initialize and run Calimero nodes
init-calimero-nodes:
	@echo "Initializing Calimero nodes..."
	@./tools/init_and_run_nodes.sh
	@echo "✓ Calimero nodes initialized and running"

# Start Calimero nodes
start-calimero:
	@echo "Starting start-calimero rule"
	@if tmux has-session -t calimero_nodes 2>/dev/null; then \
		echo "Tmux session already exists. Killing it..."; \
		tmux kill-session -t calimero_nodes; \
	fi
	@tmux new-session -d -s calimero_nodes -n "script"
	@tmux send-keys -t calimero_nodes:script "cd $(shell pwd) && clear && bash tools/init_and_run_nodes.sh" C-m
	@tmux attach -t calimero_nodes

# Clean up
clean:
	@echo "Cleaning up..."
	-dfx stop
	rm -rf .dfx
	rm -rf canister_ids.json

# Help
help:
	@echo "Calimero x ICP PoW Mining Project Makefile"
	@echo ""
	@echo "Available commands:"
	@echo "  make all          - Complete setup process"
	@echo "  make setup-icp-devnet - Set up ICP environment"
	@echo "  make init-calimero-nodes - Initialize and run Calimero nodes"
	@echo "  make clean        - Clean up environment"
	@echo "  make help         - Show this help message"

.PHONY: check-dfx check-cargo check-candid-extractor check-pnpm check-prerequisites setup-icp-devnet init-calimero-nodes build-node-app-wasm clean help all 