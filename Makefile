SHELL := bash
CC := $(shell which cargo)
PWD := $(shell pwd)
PROJECT_NAME := $(shell pwd | sed "s#.*/##")
DOCKER_IMAGE_NAME := $(shell pwd | sed "s#.*/##" | tr [:upper:] [:lower:])
BIN := rtweekend
SRC_DIR := src
LIB_DIR := lib
CARGO_TOML := Cargo.toml

all: build-shader docker-build debug

# Shader code
clean-shader:
	echo "Performing clean up of existing shaders..."
	./bin/compile_shaders.sh "clean" "${PWD}/shaders"

build-shader: clean-shader
	echo "Compiling shaders..."
	./bin/compile_shaders.sh "build" "${PWD}/shaders"

# Rust code
clean:
	find -type d -name target | while read d; do \
		rm -rvf $$d; \
	done
	sleep 1

fmt:
	$(CC) fmt

build: clean fmt
	mkdir -p bin
	$(CC) build
	cp ./target/debug/${BIN} bin

debug:
	RUST_BACKTRACE=1 OBS_VKCAPTURE=0 ENABLE_VKBASALT=0 MANGOHUD=0 ./bin/${BIN}

debug-radv:
	RUST_BACKTRACE=1 OBS_VKCAPTURE=0 ENABLE_VKBASALT=0 MANGOHUD=0 AMD_VULKAN_ICD=RADV ./bin/${BIN}

debug-full:
	RUST_BACKTRACE=full OBS_VKCAPTURE=0 ENABLE_VKBASALT=0 MANGOHUD=0 ./bin/${BIN}

run:
	./bin/${BIN}

build-linux-image:
	tar cvf docker/build.tar ${SRC_DIR} ${CARGO_TOML} ${LIB_DIR}
	docker build . -t ${DOCKER_IMAGE_NAME}/linux -f docker/Dockerfile.linux
	rm docker/build.tar

rebuild-linux-image:
	tar cvf docker/build.tar ${SRC_DIR} ${CARGO_TOML} ${LIB_DIR}
	docker build . -t ${DOCKER_IMAGE_NAME}/linux -f docker/Dockerfile.linux --no-cache
	rm docker/build.tar

docker-build: fmt
	mkdir -p bin
	docker run --rm -it -v $(shell pwd):/app ${DOCKER_IMAGE_NAME}/linux
	cp ./target/debug/${BIN} bin
