TARGET = aarch64-unknown-none
BUILDCFG = release

QEMU = qemu-system-aarch64
OBJCOPY = rust-objcopy

QEMU_MACHINE = raspi3b
QEMU_FLAGS ?= -serial stdio -display none

ARTIFACTS = target/$(TARGET)/$(BUILDCFG)
KERNEL_BIN = $(ARTIFACTS)/kernel8.img
KERNEL_ELF = $(ARTIFACTS)/onebpp

$(KERNEL_BIN): $(KERNEL_ELF)
	$(OBJCOPY) --strip-all -O binary $< $@

emu: $(KERNEL_BIN)
	$(QEMU) -machine $(QEMU_MACHINE) $(QEMU_FLAGS) -kernel $<
