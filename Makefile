CARGO := cargo
NAGA := naga
SPIRV_OPT := spirv-opt

MAKEFILE_DIR := makefiles
SHADER_DIR := shaders
SPIRV_DIR := ${SHADER_DIR}/spv

build: fmt clippy
	$(CARGO) build --release

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy --release

clean:
	$(CARGO) clean

build-shaders:
	@[ -d ${SPIRV_DIR} ] || mkdir -pv ${SPIRV_DIR}
	@WGSL="${SHADER_DIR}/vs_main.wgsl" STAGE=vert make build-shader
	@WGSL="${SHADER_DIR}/fs_main.wgsl" STAGE=frag make build-shader
	@WGSL="${SHADER_DIR}/path_tracer.wgsl" STAGE=compute make build-shader

build-shader:
	$(NAGA) --shader-stage ${STAGE} ${WGSL} $(WGSL:${SHADER_DIR}/%.wgsl=${SPIRV_DIR}/%.spv)
	$(SPIRV_OPT) $(WGSL:${SHADER_DIR}/%.wgsl=${SPIRV_DIR}/%.spv) \
		--eliminate-dead-code-aggressive \
		--simplify-instructions \
		--inline-entry-points-exhaustive \
		--convert-local-access-chains \
		--eliminate-local-single-block \
		-O \
		-o $(WGSL:${SHADER_DIR}/%.wgsl=${SPIRV_DIR}/%.spv)

docker-build:
	@TAG="linux" CMD="make build" make docker-exec

docker-build-shaders:
	@TAG="shader" CMD="make build-shaders" make docker-exec

include ${MAKEFILE_DIR}/docker.mk
