SRC_DIR := src
INCLUDE_DIR := include
BUILD_DIR := build
IMAGE_DIR := images
TARGET := $(BUILD_DIR)/rtweekend
COMPONENT_DIR :=
SRCS := $(shell find $(SRC_DIR) -type f -name "*.c")
OBJS := $(addprefix $(BUILD_DIR)/, $(patsubst %.c,%.o,$(SRCS)))
PPMS := $(wildcard $(IMAGE_DIR)/*.ppm)

CC := gcc
INCLUDES := -I$(INCLUDE_DIR) -I$(SRC_DIR)/hittable/$(INCLUDE_DIR)
CFLAGS := -Wall -Wextra -mavx2 -mfma
LDFLAGS := -lm
DEBUGGER := gdb
CONVERT := magick

ifeq (1, $(DEBUG))
	CFLAGS += -O1 -g
else
	CFLAGS += -O1
endif

.PHONY: build convert

all: clean build

debug: clean
	DEBUG=1 make build
	@$(DEBUGGER) $(TARGET)

build: $(OBJS)
	@mkdir -pv $(BUILD_DIR)
	$(CC) $(OBJS) $(LDFLAGS) -o $(TARGET)

$(BUILD_DIR)/$(SRC_DIR)/%.o: $(SRC_DIR)/%.c
	@mkdir -pv $(@D)
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

clean:
	@rm -rvf $(BUILD_DIR)

convert:
	for f in $(wildcard $(IMAGE_DIR)/*.ppm); do\
		$(CONVERT) $$f $$(echo $$f | sed "s/ppm$$/jpg/"); \
	done
