default: build run

test: build-test run

run:
	./fizzbuzz

build:
	rustc fizzbuzz.rs

build-test:
	rustc fizzbuzz.rs --test
