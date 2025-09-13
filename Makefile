help: ## Show this help
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  make %-10s %s\n", $$1, $$2}'

TASKS_DIR := tasks
MODULES_DIR := $(TASKS_DIR)/modules

WAT_FILES := $(TASKS_DIR)/init.wat
GO_FILES  := $(TASKS_DIR)/add.go
RS_FILES  := $(TASKS_DIR)/cube.rs $(TASKS_DIR)/result.rs

WASM_FILES := $(MODULES_DIR)/init.wasm \
              $(MODULES_DIR)/add.wasm \
              $(MODULES_DIR)/cube.wasm \
              $(MODULES_DIR)/result.wasm

.PHONY: modules clean run help

modules: $(WASM_FILES) ## Build all wasm modules

$(MODULES_DIR)/init.wasm: $(TASKS_DIR)/init.wat
	@mkdir -p $(MODULES_DIR)
	wat2wasm $< -o $@

$(MODULES_DIR)/add.wasm: $(TASKS_DIR)/add.go
	@mkdir -p $(MODULES_DIR)
	tinygo build -o $@ -target=wasm-unknown -no-debug -opt=z $<

$(MODULES_DIR)/cube.wasm: $(TASKS_DIR)/cube.rs
	@mkdir -p $(MODULES_DIR)
	rustc --target wasm32-unknown-unknown -O --crate-type=cdylib $< -o $@

$(MODULES_DIR)/result.wasm: $(TASKS_DIR)/result.rs
	@mkdir -p $(MODULES_DIR)
	rustc --target wasm32-unknown-unknown -O --crate-type=cdylib $< -o $@

run: modules ## Build and run the pipeline
	@echo "🚀 Running pipeline..."
	@cargo run -q

clean: ## Remove all generated wasm modules
	rm -rf $(MODULES_DIR)

