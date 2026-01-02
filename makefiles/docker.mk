DOCKERFILE_DIR := docker
BASE_IMAGE_NAME := debian
BASE_IMAGE_TAG := 13-slim
IMAGE_NAME := rtiow
BUILDER_IMAGE_TAG := builder
BUILDER_DOCKERFILE := builder.dockerfile

DOCKER := docker

build-image:
	$(DOCKER) image pull $(BASE_IMAGE_NAME):$(BASE_IMAGE_TAG)
	$(DOCKER) build . -t $(IMAGE_NAME):$(BUILDER_IMAGE_TAG) \
		-f $(DOCKERFILE_DIR)/$(BUILDER_DOCKERFILE)

docker-exec:
	$(DOCKER) run --rm -it \
		-v $(PWD):/app \
		$(IMAGE_NAME):$(TAG) bash -c "$(CMD)"
