COMPILER := $(shell which cargo)

all: build run

clean:
	$(COMPILER) clean

fmt:
	$(COMPILER) fmt

build: clean fmt
	$(COMPILER) build

test: build
	$(COMPILER) run > /dev/null

run:
	./bin/run.sh
