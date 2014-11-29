CC=gcc
LD=ld
RUSTC := rustc --cfg libc -Z no-landing-pads -O --target i686-unknown-linux-gnu
NASM=nasm
QEMU=qemu-system-i386
BUILDDIR=./build
OBJDIR=$(BUILDDIR)/obj


OBJS := $(OBJDIR)/runtime.asm.o $(OBJDIR)/main.o $(OBJDIR)/isr_wrapper.asm.o

all: $(BUILDDIR)/floppy.img

.SUFFIXES:

.SUFFIXES: .o .rs .asm

.PHONY: clean run

$(OBJDIR)/%.asm.o: src/%.asm
	mkdir -p $(OBJDIR)
	$(NASM) -f elf32 -o $@ $<

$(OBJDIR)/main.bc: src/main.rs
	mkdir -p $(OBJDIR)
	$(RUSTC) src/rust-core/core/lib.rs --out-dir $(BUILDDIR)
	$(RUSTC) --crate-type lib --emit ir -L $(BUILDDIR) -o $@ $<

$(OBJDIR)/main.o: $(OBJDIR)/main.bc
	clang -c -O2 -o $@ $<


$(BUILDDIR)/loader.bin: src/loader.asm
	mkdir -p $(BUILDDIR)
	$(NASM) -o $@ -f bin $<

$(BUILDDIR)/main.elf: src/linker.ld $(OBJS)
	mkdir -p $(BUILDDIR)
	$(LD) -Map=$(BUILDDIR)/linker.map -o $@ -T $^ "-(" build/libcore-2e829c2f-0.0.rlib "-)"

$(BUILDDIR)/main.bin: $(BUILDDIR)/main.elf
	mkdir -p $(BUILDDIR)
	objcopy -O binary $< $@

$(BUILDDIR)/floppy.img: $(BUILDDIR)/loader.bin $(BUILDDIR)/main.bin
	cat $^ > $@

run: $(BUILDDIR)/floppy.img
	$(QEMU) -serial stdio -fda $<

debug: $(BUILDDIR)/floppy.img
	$(QEMU) -monitor stdio -fda $<

clean:
	-rm -rf $(BUILDDIR)
