COMPILER := "$(shell which go)"
BIN_DIR := "./rtweekend"

fmt:
	(cd $(BIN_DIR) && $(COMPILER) fmt)

tidy: fmt
	$(COMPILER) mod tidy

build: tidy
	cd $(BIN_DIR) && go build ./
