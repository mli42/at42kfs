NAME = build/kfsos.iso
LINKER_SCRIPT = linker.ld
KERNEL_BIN = dist/boot/kfsos.bin

DC = docker compose

ASM_OBJS_DIR = .obj
ASM_SRCS = src/multiboot_header.asm
ASM_OBJS = ${addprefix ${ASM_OBJS_DIR}/, ${ASM_SRCS:src/%.asm=%.o}}
ASM_FLAGS = -f elf32

RUST_SRCS = src/main.rs src/vga_buffer.rs src/allocator.rs src/gdt.rs
RUST_BUILD = target/i386-kfsos/debug/libkfsos.a

.PHONY: all
all: $(NAME)

.PHONY: run
run:
	qemu-system-i386 -drive format=raw,file=$(NAME)

$(RUST_BUILD): $(RUST_SRCS)
	# Compile rust
	cargo build

$(KERNEL_BIN): $(ASM_OBJS) $(RUST_BUILD) $(LINKER_SCRIPT)
	# Link Kernel & Multiboot header
	ld --nmagic --output=$(KERNEL_BIN) --script=$(LINKER_SCRIPT) $(ASM_OBJS) $(RUST_BUILD)

$(NAME): $(KERNEL_BIN) dist/boot/grub/grub.cfg
	# Build ISO
	mkdir -p ./build
	grub-mkrescue -o $(NAME) dist
	chmod 777 $(NAME)

$(ASM_OBJS_DIR)/%.o: src/%.asm $(ASM_OBJS_DIR)
	nasm $(ASM_FLAGS) $< -o $@

$(ASM_OBJS_DIR):
	mkdir -p $(ASM_OBJS_DIR)

.PHONY: clean
clean:
	rm -rf ./target
	rm -rf ./build
	rm -rf $(ASM_OBJS_DIR)
	rm -f $(KERNEL_BIN)

.PHONY: fclean
fclean: clean
	rm -f $(NAME)

.PHONY: re
re: fclean all

.PHONY: docker.up
docker.up:
	$(DC) up -d

.PHONY: docker.build
docker.build:
	$(DC) exec kfs make

.PHONY: docker.fclean
docker.fclean:
	$(DC) exec kfs make fclean

.PHONY: docker.re
docker.re:
	$(DC) exec kfs make re

.PHONY: docker.down
docker.down:
	$(DC) down
