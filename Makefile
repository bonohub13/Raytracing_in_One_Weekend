all: build run

clean:
	(cd build && make clean)
	$(shell [ -f bin/rtweekend ] && rm bin/rtweekend)

build: clean
	(cd build && cmake .. && make)
	mv build/src/rtweekend bin

run:
	./bin/run.sh
