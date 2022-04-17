COMPILER := $(shell which cargo)

all: build

clean:
	$(COMPILER) clean

build: clean
	$(COMPILER) build

test: build
	$(COMPILER) run > /dev/null

run: build
	./bin/run.sh
