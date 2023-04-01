NAME=build/lsarrazios.iso

KERNEL_BIN=dist/boot/lsarrazios.bin

ASM_OBJS= obj/multiboot_header.o
ASM_SRCS= src/multiboot_header.asm
RUST_SRCS= src/main.rs src/vga_buffer.rs

ASM_FLAGS= -f elf32
all: $(NAME)

install:
	# Install RUST and set nightly version
	apt install build-essential
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup override set nightly
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

	# Install GRUB dependencies
	apt install grub-pc-bin grub-common xorriso qemu-system

run:
	qemu-system-x86_64 -drive format=raw,file=$(NAME)

$(KERNEL_BIN): $(ASM_OBJS) $(RUST_SRCS)
	# Build KERNEL_BIN
	cargo build

	# Link Kernel & Multiboot header
	ld --nmagic --output=$(KERNEL_BIN) --script=./linker.ld $(ASM_OBJS) ./target/i386-lsarrazios/debug/liblsarrazios.a

$(NAME): $(KERNEL_BIN) dist/boot/grub/grub.cfg
	# Build ISO
	mkdir -p ./build
	grub-mkrescue -o $(NAME) dist


obj/%.o: src/%.asm
	mkdir -p ./obj
	nasm $(ASM_FLAGS) $< -o $@