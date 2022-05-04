COMPILER := $(shell which go)
DIRECTORIES := $(shell find -type f -name "*.go" | sed "s;/[A-Z|a-z|0-9|_]*\.go;;" | uniq)
SOURCE_DIR := ./rtweekend
BIN_DIR := bin
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
	(cd $(SOURCE_DIR) && go build -o $(PROJECT_ROOT)/$(BIN_DIR) ./)

run:
	$(PROJECT_ROOT)/$(BIN_DIR)/run.sh
