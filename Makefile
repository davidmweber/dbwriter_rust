# HELP. Thanks to https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
DOCKER_REPO = ghcr.io
NAME = dbwriter_rust
VERSION = $(shell sed -nE 's/^version\s*=\s*"([^"]+)"/\1/p' Cargo.toml)
AWS_DOCKER_REPO = 008971667931.dkr.ecr.eu-west-1.amazonaws.com
AWS_NAME = rust-dragrace
help: ## This help.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

release: ## Create a release build
	cargo build --release

container: release ## Build the container
	docker build --tag $(NAME):${VERSION} --tag $(NAME):latest\
		--tag $(AWS_DOCKER_REPO)/$(AWS_NAME):$(VERSION) --tag $(AWS_DOCKER_REPO)/$(AWS_NAME):latest .

ecr_push: ## Push the container to ECR
	aws ecr get-login-password --region eu-west-1 --profile sandbox | docker login --username AWS --password-stdin $(AWS_DOCKER_REPO)
	docker push $(AWS_DOCKER_REPO)/$(AWS_NAME):$(VERSION)
	docker push $(AWS_DOCKER_REPO)/$(AWS_NAME):latest


ecs_deploy:
	aws ecs update-service \
		--task-definition energy_platform-api-$(ENVIRONMENT) \
		--cluster energy_platform-cluster-$(ENVIRONMENT) \
		--service energy-platform-api-$(ENVIRONMENT) --force-new-deployment | jq ".service.deployments[0].rolloutState"


clean: ## Clean the build
	cargo clean
