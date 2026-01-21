SRC_DIR := src
INCLUDE_DIR := include
BUILD_DIR := build
IMAGE_DIR := images
MAKEFILES_DIR := makefiles
TARGET := $(BUILD_DIR)/rtweekend
COMPONENT_DIR :=
SRCS := $(shell find $(SRC_DIR) -type f -name "*.c")
OBJS := $(addprefix $(BUILD_DIR)/, $(patsubst %.c,%.o,$(SRCS)))
ASMS := $(addprefix $(BUILD_DIR)/, $(patsubst %.c,%.s,$(SRCS)))
PPMS := $(wildcard $(IMAGE_DIR)/*.ppm)

CC := gcc
INCLUDES := -I$(INCLUDE_DIR) \
			-I$(SRC_DIR)/vec3/$(INCLUDE_DIR) \
			-I$(SRC_DIR)/hittable/$(INCLUDE_DIR) \
			-I$(SRC_DIR)/material/$(INCLUDE_DIR) \
			-I$(SRC_DIR)/scenes/$(INCLUDE_DIR)
CFLAGS := -Wall -Wextra -O3
LDFLAGS := -lm
DEBUGGER := gdb
CONVERT := magick
FORCE_FALLBACK ?= 0

ifeq (1, $(DEBUG))
	CFLAGS += -g
endif

ifeq (1, $(FORCE_FALLBACK))
	CFLAGS += -DFORCE_FALLBACK
else
	CFLAGS := -march=native
endif

.PHONY: build convert

all: clean build

docker-build:
	FORCE_FALLBACK=$(FORCE_FALLBACK) TAG=builder CMD="make all" make docker-exec

docker-run:
	FORCE_FALLBACK=$(FORCE_FALLBACK) TAG=builder CMD="$(TARGET)" make docker-exec

debug: clean
	FORCE_FALLBACK=$(FORCE_FALLBACK) DEBUG=1 make build
	@$(DEBUGGER) $(TARGET)

build: $(OBJS) $(ASMS)
	@mkdir -pv $(BUILD_DIR)
	$(CC) $(OBJS) $(LDFLAGS) -o $(TARGET)

$(BUILD_DIR)/$(SRC_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -pv $(@D)
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

$(BUILD_DIR)/$(SRC_DIR)/%.s: $(SRC_DIR)/%.c
	@mkdir -pv $(@D)
	$(CC) -S $(CFLAGS) $(INCLUDES) -c $< -o $@

clean:
	@rm -rvf $(BUILD_DIR)

convert:
	for f in $(wildcard $(IMAGE_DIR)/*.ppm); do\
		$(CONVERT) $$f $$(echo $$f | sed "s/ppm$$/jpg/"); \
	done

include $(MAKEFILES_DIR)/docker.mk
