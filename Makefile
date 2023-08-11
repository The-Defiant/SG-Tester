build: #:: build optimized executable
	cargo build -r

test: #:: test cargo project
	cargo test

lint: #:: lint cargo project
	cargo clippy

format: #:: format cqrgo project
	cargo fmt

build-image: #:: build image with latest version of sgt
	docker build -t project-defiant/sgt:latest -f deployment/dockerfile .

help:
	@printf "\033[0;32m************************** Help for commands **********************\033[38;5;38m\n"
	@grep -E '^[a-zA-Z_-]+:.*?#::.*' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":[^:]*?#:: "}; {printf "  \033[38;5;69m%s\033[38;5;38m::%s\033[0m\n", $$1, $$2}' | column -t -s "::"
