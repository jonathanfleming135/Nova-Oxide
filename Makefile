# Variables
WORKING_DIR = /home/pi/nova-oxide
CARGO = /home/pi/.cargo/bin/cargo
BINARY_PATH = /home/pi/nova-oxide/target/debug/nova-oxide

# Build the project
build:
	rsync -r ./* pi:$(WORKING_DIR) --exclude=target/* --exclude=ws281x-rpi/target/*
	ssh pi "cd $(WORKING_DIR) && $(CARGO) build"

# Run the project
run:
	ssh -t pi "sudo $(BINARY_PATH)"

# Build and run the project
all: build run

# Clean the build artifacts
clean:
	rm -rf target
	ssh pi "cd $(WORKING_DIR) && rm -rf target"

# Phony targets
.PHONY: build run clean
