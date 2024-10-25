# Variables
WORKING_DIR = /home/pi/nova-oxide
CARGO = /home/pi/.cargo/bin/cargo
BINARY_PATH = $(WORKING_DIR)/target/debug/nova-oxide
TEST_BINARY = $(WORKING_DIR)/test-binary.txt

# Build the project
build:
	rsync -r ./* pi:$(WORKING_DIR) --exclude=target/* --exclude=ws281x-rpi/target/*
	ssh pi "cd $(WORKING_DIR) && $(CARGO) build"

# Run the project
run:
	ssh -t pi "sudo $(BINARY_PATH)"

# Run the tests
test: build
	ssh pi "cd $(WORKING_DIR) \
			&& $(CARGO) test --no-run --message-format=json -q \
			| jq -r 'select(.executable) \
			| .executable' \
			| head -n 1 > $(BINARY_PATH)"
	ssh -t pi "cd $(WORKING_DIR) && cat test-binary.txt | sudo bash"

# Build and run the project
all: build run

# Clean the build artifacts
clean:
	rm -rf target
	ssh pi "cd $(WORKING_DIR) && rm -rf target"

# Phony targets
.PHONY: build run test all clean
