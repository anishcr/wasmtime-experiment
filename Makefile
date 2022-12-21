.DEFAULT_GOAL := all

## help : run this help message
.PHONY: help
help:
	@echo 'Usage:'
	@sed -n 's/^##//p' ${MAKEFILE_LIST} | column -t -s ':' | sed -e 's/^/ /'

## runtime : build and run the runtime. Ensure that the wasm app is built before this
.PHONY: runtime
runtime:
	@echo 'Building and running runtime'
	cd runtime && cargo run

## app : Build the wasm module and copy the wasm file to the runtime directory
.PHONY: app
app:
	@echo 'Building app'
	cd app && cargo wasi build && cp -nf target/wasm32-wasi/debug/app.wasm ../runtime/

## all : build all and run the runtime 
.PHONY: all
all: app runtime
