COMPILER := $(shell which cargo)

all: build

clean:
	$(COMPILER) clean

fmt:
	$(COMPILER) fmt

build: clean fmt
	$(COMPILER) build

test: build
	$(COMPILER) run > /dev/null

run: build
	./bin/run.sh
