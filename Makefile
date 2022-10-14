SHELL := bash
CC := $(shell which cargo)
PWD := $(shell pwd)
DOCKER_IMAGE_NAME := rt_in_one_weekend-vk
BIN_NAME := rtweekend

all: docker-build run

# Shader code
clean-shader:
	echo "Performing clean up of existing shaders..."
	./bin/compile_shaders.sh "clean" "${PWD}/shaders"

build-shader: clean-shader
	echo "Compiling shaders..."
	./bin/compile_shaders.sh "build" "${PWD}/shaders"

# Rust code
clean:
	$(CC) clean

fmt:
	$(CC) fmt

build: fmt clean
	mkdir -p bin
	$(CC) build
	cp ./target/debug/${BIN_NAME} bin

run:
	[ -d "/tmp" ] \
		&& ([ -d "/tmp/${BIN_NAME}" ] || mkdir "/tmp/${BIN_NAME}") \
		&& OBS_VKCAPTURE=0 ENABLE_VKBASALT=0 MANGOHUD=0 ./bin/${BIN_NAME} 2>&1 \
			| tee "/tmp/${BIN_NAME}/$(shell date +'%Y%m%d-%H%M%S').log"

run-with-mangohud:
	[ -d "/tmp" ] \
		&& ([ -d "/tmp/${BIN_NAME}" ] || mkdir "/tmp/${BIN_NAME}") \
		&& OBS_VKCAPTURE=0 ENABLE_VKBASALT=0 MANGOHUD=1 ./bin/${BIN_NAME} 2>&1 \
			| tee "/tmp/${BIN_NAME}/$(shell date +'%Y%m%d-%H%M%S').log"

rebuild-linux-image:
	cp Cargo.toml docker
	docker build . -t ${DOCKER_IMAGE_NAME}/linux -f docker/Dockerfile.linux --no-cache
	rm docker/Cargo.toml

rebuild-all-images: rebuild-linux-image

docker-build: build-shader clean
	mkdir -p bin
	docker run --rm -it -v $(shell pwd):/app ${DOCKER_IMAGE_NAME}/linux
	cp ./target/debug/${BIN_NAME} bin
