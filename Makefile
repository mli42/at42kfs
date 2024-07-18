NAME = build/kfsos.iso
LINKER_SCRIPT = linker.ld
KERNEL_BIN = dist/boot/kfsos.bin

DC = docker compose

ASM_OBJS_DIR = .obj
ASM_SRCS = src/multiboot_header.asm
ASM_OBJS = ${addprefix ${ASM_OBJS_DIR}/, ${ASM_SRCS:src/%.asm=%.o}}
ASM_FLAGS = -f elf32

# RUST_MODE is either `debug` or `release`
RUST_MODE ?= debug
RUST_SRCS = ${addprefix src/, \
	main.rs \
	panic.rs \
	${addprefix gdt/, \
		mod.rs \
	} \
	${addprefix interrupts/, \
		mod.rs \
		idt.rs \
		isr.rs \
		pic8259.rs \
	} \
	${addprefix io/, \
		mod.rs \
	} \
	${addprefix vga_buffer/, \
		mod.rs \
	} \
	${addprefix keyboard/, \
		mod.rs \
		keymap_us.rs \
		keymap_fr.rs \
	} \
	${addprefix cli/, \
		mod.rs \
		commands.rs \
	} \
	${addprefix utils/, \
		mod.rs \
		${addprefix asm/, \
			mod.rs \
		} \
	} \
}
RUST_CONFIG = Cargo.toml
RUST_BUILD = target/i386-kfsos/$(RUST_MODE)/libkfsos.a

ifeq ($(RUST_MODE), release)
	RUST_FLAGS = --release
endif

.PHONY: all
all: $(NAME)

.PHONY: run
run:
	qemu-system-i386 -drive format=raw,file=$(NAME) -no-reboot -d int

$(RUST_BUILD): $(RUST_SRCS) $(RUST_CONFIG)
	# Compile rust
	cargo build $(RUST_FLAGS)

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
	$(DC) exec kfs make RUST_MODE=$(RUST_MODE)

.PHONY: docker.fclean
docker.fclean:
	$(DC) exec kfs make fclean RUST_MODE=$(RUST_MODE)

.PHONY: docker.re
docker.re:
	$(DC) exec kfs make re RUST_MODE=$(RUST_MODE)

.PHONY: docker.down
docker.down:
	$(DC) down
