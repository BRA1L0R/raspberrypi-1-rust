ARMTOOL ?= arm-none-eabi
BUILD ?= release

build:
	cargo build --$(BUILD)

bin: build
	$(ARMTOOL)-objcopy target/rpi/$(BUILD)/rpi-video -O binary target/kernel.bin 

qemu:
	qemu-system-arm -M raspi2 -kernel target/kernel.bin -d in_asm

dev: bin qemu