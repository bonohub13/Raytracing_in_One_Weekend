SHELL := bash
CARGO := cargo
PWD := $(shell pwd)
PROJECT_NAME := $(shell pwd | sed "s#.*/##")
DOCKER_IMAGE_NAME := $(shell pwd | sed "s#.*/##" | tr [:upper:] [:lower:])

all: build run

# Rust code
prepare:
	@[ -d images ] || mkdir -v images

clean:
	@$(CARGO) clean
	@$(CARGO) clean --package=rtiow

fmt:
	@$(CARGO) fmt

build: prepare fmt
	@$(CARGO) build --release

run: prepare fmt
	@$(CARGO) run --release

test: fmt
	@$(CARGO) test
	@$(CARGO) test --package=rtiow

build-offline: prepare fmt
	@$(CARGO) build --release --offline

run-offline: prepare fmt
	@$(CARGO) run --release --offline

test-offline: fmt
	@$(CARGO) test
	@$(CARGO) test --package=rtiow --offline

rebuild-linux-image:
	@cp -v Cargo.toml docker
	@docker build . -t ${DOCKER_IMAGE_NAME}/linux -f docker/Dockerfile.linux --no-cache
	@rm docker/Cargo.toml

docker-build: prepare fmt clean
	@docker run --rm -it -v $(shell pwd):/app ${DOCKER_IMAGE_NAME}/linux
