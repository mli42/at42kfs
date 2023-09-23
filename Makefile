NAME=build/kfsos.iso

KERNEL_BIN=dist/boot/kfsos.bin

ASM_OBJS= obj/multiboot_header.o
ASM_SRCS= src/multiboot_header.asm
RUST_SRCS= src/main.rs src/vga_buffer.rs src/gdt.rs

LINKER_SCRIPT=./linker.ld

ASM_FLAGS= -f elf32
all: $(NAME)

run:
	qemu-system-x86_64 -drive format=raw,file=$(NAME)

$(KERNEL_BIN): $(ASM_OBJS) $(RUST_SRCS) $(LINKER_SCRIPT)
	# Build KERNEL_BIN
	cargo build

	# Link Kernel & Multiboot header
	ld --nmagic --output=$(KERNEL_BIN) --script=$(LINKER_SCRIPT) $(ASM_OBJS) ./target/i386-kfsos/debug/libkfsos.a

$(NAME): $(KERNEL_BIN) dist/boot/grub/grub.cfg
	# Build ISO
	mkdir -p ./build
	grub-mkrescue -o $(NAME) dist
	chmod 777 $(NAME)

clean:
	rm -rf ./target
	rm -rf ./build
	rm -rf ./obj
	rm -f $(KERNEL_BIN)

fclean: clean
	rm -f $(NAME)

re: fclean all

obj/%.o: src/%.asm
	mkdir -p ./obj
	nasm $(ASM_FLAGS) $< -o $@
