COMPILER := $(shell which go)
DIRECTORIES := $(shell find -type f -name "*.go" | sed "s;/[A-Z|a-z|0-9]*\.go;;" | uniq)
BIN_DIR := ./rtweekend

fmt:
	for dir in $(DIRECTORIES); do \
		(cd $$dir && $(COMPILER) fmt); \
	done

tidy: fmt
	$(COMPILER) mod tidy

build: tidy
	cd $(BIN_DIR) && go build ./
