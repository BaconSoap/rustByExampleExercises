RUSTC := rustc
CARGS ?= -g
TARGET := part1

.PHONY: all
all:
	RUSTC $(CARGS) $(TARGET).rs -o bin/$(TARGET)
.PHONY: run
run: all
	./bin/$(TARGET)
.PHONY: debug
debug: all
	gdb ./bin/$(TARGET)
