RUSTC := rustc
TARGET := part1

.PHONY: all
all:
	RUSTC $(TARGET).rs
.PHONY: run
run: all
	./$(TARGET)
