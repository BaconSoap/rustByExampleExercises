RUSTC := rustc
TARGET := part1

.PHONY: all
all:
	RUSTC $(TARGET).rs -o bin/$(TARGET)
.PHONY: run
run: all
	./bin/$(TARGET)
