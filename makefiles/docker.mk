DOCKER := docker
DOCKER_DIR := docker
CARGO_REGISTRY_DIR := $(HOME)/.cargo/registry

build-linux-image:
	@$(DOCKER) build . -t ${DOCKER_IMAGE_NAME}/linux -f $(DOCKER)/Dockerfile.linux --no-cache

docker-exec: prepare fmt clean
	@$(DOCKER) run --rm -it \
		-v $(PWD):/app \
		-v $(CARGO_REGISTRY_DIR):/usr/local/cargo/registry \
		${DOCKER_IMAGE_NAME}/linux \
		bash -c "$(CMD)"
