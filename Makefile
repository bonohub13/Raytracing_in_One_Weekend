COMPILER := $(shell which go)
DIRECTORIES := $(shell find -type f -name "*.go" | sed "s;/[A-Z|a-z|0-9|_]*\.go;;" | uniq)
BIN_DIR := bin
BIN := rtweekend
PROJECT_ROOT := $(shell pwd)

all: build run

fmt:
	for dir in $(DIRECTORIES); do \
		(cd $$dir && $(COMPILER) fmt); \
	done

tidy: fmt
	$(COMPILER) mod tidy

clean:
	$(COMPILER) clean

build: clean tidy
	mkdir -p $(PROJECT_ROOT)/$(BIN_DIR)
	go build -o $(PROJECT_ROOT)/$(BIN_DIR)/${BIN} ./

run:
	$(PROJECT_ROOT)/$(BIN_DIR)/run.sh
