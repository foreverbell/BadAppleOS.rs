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

build/vdata.bin:
	python resource/encode.py resource/video.txt build/vdata.bin

build/vdata.o: build/vdata.bin
	objcopy -B i386 -I binary -O elf32-i386 build/vdata.bin build/vdata.o

build/begin.o: src/begin.asm
	$(AS) -o build/begin.o $(ASFLAGS) src/begin.asm

build/descriptor_flush.o: src/krnl/descriptor_flush.asm
	$(AS) -o $@ $(ASFLAGS) src/krnl/descriptor_flush.asm

build/irq_handler.o: src/krnl/irq_handler.asm
	$(AS) -o $@ $(ASFLAGS) src/krnl/irq_handler.asm

build/isr_handler.o: src/krnl/isr_handler.asm
	$(AS) -o $@ $(ASFLAGS) src/krnl/isr_handler.asm

AS_OBJECTS := \
	build/begin.o \
	build/descriptor_flush.o \
	build/irq_handler.o \
	build/isr_handler.o

kernel:
	RUST_TARGET_PATH=$(shell pwd) xargo build --target=i386-unknown-none --release

build/kernel.elf: kernel build $(AS_OBJECTS) build/vdata.o
	$(LD) -o build/kernel.elf $(LDFLAGS) \
		$(AS_OBJECTS) \
		build/vdata.o \
		target/i386-unknown-none/release/libBadAppleOS_rs.a

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
