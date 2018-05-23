arch ?= x86_64
kern := kernel-$(arch).bin
kernel := build/$(kern)
iso := build/os-$(arch).iso
target ?= $(arch)-os
#  this maybe an error
rust_os := target/$(target)/debug/libos.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel 

all: $(kernel)

clean:
	@rm -r build
	@xargo clean

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/$(kern)
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 

# @rm -r build/isofiles

$(kernel):  kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n  --gc-sections -T $(linker_script) -o $(kernel) \
	$(assembly_object_files) $(rust_os)

kernel:
	@RUST_TARGET_PATH=$(shell pwd) xargo build --target $(target)

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
