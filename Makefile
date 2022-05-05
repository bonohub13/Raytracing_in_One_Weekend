all: build run

clean:
	$(shell [ -d build ] && rm -rf build)
	$(shell [ -d images ] && find images -type f -name "*~" | xargs rm -f)
	mkdir -p build
	$(shell [ -f bin/rtweekend ] && rm bin/rtweekend)

build: clean
	(cd build && cmake .. && make)
	mv build/src/rtweekend bin

run:
	./bin/run.sh
