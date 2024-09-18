# Compiler settings
CC := gcc
CFLAGS := -Wall -Wextra -std=c11 -g

# Directories
SRC_DIR := src
OBJ_DIR := obj
BIN_DIR := bin

# Source files and object files
SRCS := $(wildcard $(SRC_DIR)/*.c)
OBJS := $(SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)

# Main target
TARGET := $(BIN_DIR)/myapp

# Phony targets
.PHONY: all clean debug release

# Default target
all: $(TARGET)

# Linking
$(TARGET): $(OBJS) | $(BIN_DIR)
	@echo "Linking $@"
	@$(CC) $(CFLAGS) $^ -o $@

# Compiling
$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	@echo "Compiling $<"
	@$(CC) $(CFLAGS) -c $< -o $@

# Create directories
$(BIN_DIR) $(OBJ_DIR):
	@mkdir -p $@

# Clean up
clean:
	@echo "Cleaning up..."
	@rm -rf $(OBJ_DIR) $(BIN_DIR)

# Debug build
debug: CFLAGS += -DDEBUG -g
debug: all

# Release build
release: CFLAGS += -O2 -DNDEBUG
release: all

# Include dependencies
-include $(OBJS:.o=.d)

# Generate dependencies
$(OBJ_DIR)/%.d: $(SRC_DIR)/%.c | $(OBJ_DIR)
	@$(CC) $(CFLAGS) -MM -MT $(@:.d=.o) $< > $@

# Help target
help:
	@echo "Available targets:"
	@echo "  all     : Build the main target (default)"
	@echo "  clean   : Remove object files and binaries"
	@echo "  debug   : Build with debug symbols"
	@echo "  release : Build with optimizations"
	@echo "  help    : Show this help message"

# Version information
VERSION := 1.0.0
version:
	@echo "MyApp version $(VERSION)"