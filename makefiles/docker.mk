DOCKER := /usr/bin/docker

IMAGE_NAME := rtiow
BASE_IMAGE_TAG := base
SHADER_IMAGE_TAG := shader
LINUX_IMAGE_TAG := linux

DOCKER_DIR := docker
CARGO_REGISTRY_DIR := ${HOME}/.cargo/registry

build-all-images: build-base build-linux

update-dependency:
	$(DOCKER) image pull rust:latest

build-image:
	$(DOCKER) build . -t ${IMAGE_NAME}:${TAG} \
		-f ${DOCKER_DIR}/${TAG}.dockerfile

build-image_base: update-dependency
	@TAG=${BASE_IMAGE_TAG} make build-image

build-image_shader: update-dependency
	@TAG=${SHADER_IMAGE_TAG} make build-image

build-image_linux: build-image_base
	@TAG=${LINUX_IMAGE_TAG} make build-image

docker-exec:
	$(DOCKER) run --rm -it \
		-v ${PWD}:/app \
		-v ${CARGO_REGISTRY_DIR}:/usr/local/cargo/registry \
		${IMAGE_NAME}:${TAG} \
		bash -c "$(CMD)"
