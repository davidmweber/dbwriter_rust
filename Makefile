# HELP. Thanks to https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
DOCKER_REPO = ghcr.io
NAME = dbwriter_rust
VERSION = $(shell sed -nE 's/^version\s*=\s*"([^"]+)"/\1/p' Cargo.toml)

help: ## This help.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

release: ## Create a release build
	cargo build --release

container: release ## Build the container
	docker build --tag $(NAME):${VERSION} --tag $(NAME):latest .


clean: ## Clean the build
	cargo clean
