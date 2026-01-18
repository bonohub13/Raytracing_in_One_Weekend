SHELL := bash
CARGO := cargo
PWD := $(shell pwd)
PROJECT_NAME := $(shell pwd | sed "s#.*/##")
DOCKER_IMAGE_NAME := $(shell pwd | sed "s#.*/##" | tr [:upper:] [:lower:])

all: build run

docker-build:
	CMD="make build" make docker-exec

docker-run:
	CMD="make run" make docker-exec

# Rust code
prepare:
	@[ -d images ] || mkdir -v images

resources:
	@chmod +x scripts/resources.sh
	@./scripts/resources.sh

clean:
	@$(CARGO) clean
	@$(CARGO) clean --package=rtiow

fmt:
	@$(CARGO) fmt

build: prepare fmt
	@$(CARGO) build --release

run: prepare fmt
	@$(CARGO) run --release --bin=raytracing

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

pkg:
	@[ -d images ]
	@command -v pigz && tar -I pigz -cvf past_renders.tar.gz images || tar czvf past_renders.tar.gz images

extract:
	@command -v pigz \
		&& tar -I pigz -xvf past_renders.tar.gz \
		|| tar xzvf past_renders.tar.gz

include makefiles/docker.mk
