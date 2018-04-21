ASFLAGS := -f elf
LDFLAGS := -n -T src/link.ld -m elf_i386
OCFLAGS := -O binary

AS := nasm
LD := ld
OC := objcopy

ISO := build/BadApple.iso

all: build/kernel.bin

docker:
	docker build -t=grub:latest docker

build:
	mkdir -p build

build/begin.o: src/begin.asm
	$(AS) -o build/begin.o $(ASFLAGS) src/begin.asm

build/descriptor_flush.o: src/krnl/descriptor_flush.asm
	$(AS) -o build/descriptor_flush.o $(ASFLAGS) src/krnl/descriptor_flush.asm

kernel:
	RUST_TARGET_PATH=$(shell pwd) xargo build --target=i686-unknown-none

build/kernel.elf: kernel build build/begin.o build/descriptor_flush.o
	$(LD) -o build/kernel.elf $(LDFLAGS) \
		build/begin.o \
		build/descriptor_flush.o \
		target/i686-unknown-none/debug/libBadAppleOS_rs.a

build/kernel.bin: build/kernel.elf
	$(OC) $(OCFLAGS) build/kernel.elf build/kernel.bin

iso: build/kernel.elf
	mkdir -p build/iso/boot/grub
	cp grub.cfg build/iso/boot/grub/grub.cfg
	cp build/kernel.elf build/iso/boot/kernel.elf
	docker run -it --rm -v $(shell pwd):$(shell pwd) -w $(shell pwd) -u `id -u $(shell USER)` grub:latest -o $(ISO) build/iso

dump: build/kernel.bin
	ndisasm -b32 -oc0010000h build/kernel.bin > build/dump.txt

qemu: iso
	qemu-system-i386 -cdrom $(ISO)

debug: iso
	qemu-system-i386 -d int -no-reboot -cdrom $(ISO)

clean:
	rm -rf build/ target/

.PHONY: kernel docker iso dump qemu debug clean
