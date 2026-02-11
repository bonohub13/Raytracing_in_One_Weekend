CARGO := cargo
SLANGC := slangc
SPIRV_OPT := spirv-opt

MAKEFILE_DIR := makefiles
SHADER_DIR := shaders
SPIRV_DIR := ${SHADER_DIR}/spv

SLANG_FLAG := -I shaders \
			  -target spirv \
			  -profile sm_6_0
SPIRV_OPT_FLAG := --eliminate-dead-code-aggressive \
				  --simplify-instructions \
				  --inline-entry-points-exhaustive \
				  --convert-local-access-chains \
				  --eliminate-local-single-block \
				  -O

all: build build-shaders

build: fmt clippy
	$(CARGO) build --release

debug: fmt
	$(CARGO) clippy
	$(CARGO) build

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy --release

clean:
	$(CARGO) clean

build-shaders:
	@[ -d ${SPIRV_DIR} ] || mkdir -pv ${SPIRV_DIR}
	@SLANG="${SHADER_DIR}/vertex.slang" ENTRY="main" make build-shader
	@SLANG="${SHADER_DIR}/fragment.slang" ENTRY="main" make build-shader
	@SLANG="${SHADER_DIR}/path_tracer.slang" ENTRY="main" make build-shader

build-shader:
	$(SLANGC) ${SLANG} ${SLANG_FLAG} -entry ${ENTRY} -o $(SLANG:${SHADER_DIR}/%.slang=${SPIRV_DIR}/%.spv)
	$(SPIRV_OPT) $(SLANG:${SHADER_DIR}/%.slang=${SPIRV_DIR}/%.spv) ${SPIRV_OPT_FLAG} \
		-o $(SLANG:${SHADER_DIR}/%.slang=${SPIRV_DIR}/%.spv)

docker-build:
	@TAG="linux" CMD="make build" make docker-exec

docker-debug:
	@TAG="linux" CMD="make debug" make docker-exec

docker-build-shaders:
	@TAG="shader" CMD="make build-shaders" make docker-exec

docker-build-all: docker-build docker-build-shaders

include ${MAKEFILE_DIR}/docker.mk
