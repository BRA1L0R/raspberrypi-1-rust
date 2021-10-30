> This repository is aimed to provide a starting point to whoever is trying to build a BCM2835 program from scratch. This repository contains linker scripts and start up codes for it, as well as practical examples of how to work with the Mini UART interface and the frame-buffer graphics. It also contains a basic Rust implementation of the Bresenham' line drawing algorithm.

# Learning resources

If you are here you are probably looking for somewhere to start your baremetal journey. Here are a couple of links that really helped me:

- [Dwelch67's pi tutorial](https://github.com/dwelch67/raspberrypi):
  - [Baremetal introduction](https://github.com/dwelch67/raspberrypi/blob/master/baremetal/README)
  - [BSS zeroing explaination](https://github.com/dwelch67/raspberrypi/blob/master/bssdata/README)
- [BCM283' framebuffer](https://elinux.org/RPi_Framebuffer) (unofficial documentation)
- [Jsandler18's tutorial](https://jsandler18.github.io/)
- [Raspberry Pi 4 Rust Os](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials) (too many architectural differences)

# Compiling

## Requirements

The systems running BCM2835 are:

- Raspberry Pi B
- Raspberry Pi B2
- Raspberry Pi A+
- Raspberry Pi B+
- Raspberry Pi Zero

> NOTE:
> I haven't tested any of the aforementioned machines, except for the Pi B+.

You will need to have installed the latest version of the `arm-none-eabi-gcc`
toolchain: this program uses it for linking. Check out rpi.json for
more informations on how the target of this program is structured. You will also need `make`.

## Using `make`:

```sh
make bin
```

This will output a `target/kernel.bin` bin file, ready to be used on the BCM2835.

## Preparing the SD Card:

- Get a **compatible SD Card**. I have a generic Toshiba 4GB SDHC
- **Create** a new `DOS` **partition table**
- **Partition** a `W95 FAT32` partition of your size choice
- **Get** from [here](https://github.com/raspberrypi/firmware/tree/master/boot) these files:
  - bootcode.bin
  - fixup.dat
  - start.elf
- **Move** the files in the newly created FAT32 partition
- **Create** a `config.txt` file with these contents:

```ini
enable_uart=1
uart_2ndstage=1
kernel=kernel.bin
```

- **Move** the previously compiled `target/kernel.bin` on the SD card
- **Unmount** the partition and **eject** the SD card
- **_Profit!_**
