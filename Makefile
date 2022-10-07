SHELL := bash
CC := $(shell which cargo)
PWD := $(shell pwd)

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
	cp ./target/debug/rtweekend bin

run:
	[ -d "/tmp" ] \
		&& ([ -d "/tmp/rt_weekend" ] || mkdir "/tmp/rt_weekend") \
		&& OBS_VKCAPTURE=0 ENABLE_VKBASALT=0 MANGOHUD=0 ./bin/rt_weekend 2>&1 \
			| tee "/tmp/rt_weekend/$(shell date +'%Y%m%d-%H%M%S').log"

run-with-mangohud:
	OBS_VKCAPTURE=0 ENABLE_VKBASALT=0 MANGOHUD=1 ./bin/rt_weekend 2>&1 | tee "/tmp/$(shell date +'%Y%m%d-%H%M%S').log"

rebuild-win64-image:
	docker build . -t ofv/windows -f docker/Dockerfile.windows --no-cache

rebuild-linux-image:
	cp Cargo.toml docker
	docker build . -t ofv/linux -f docker/Dockerfile.linux --no-cache
	rm docker/Cargo.toml

rebuild-all-images: rebuild-linux-image

docker-build: build-shader clean
	mkdir -p bin
	docker run --rm -it -v $(shell pwd):/app ofv/linux
	cp ./target/debug/rt_weekend bin
