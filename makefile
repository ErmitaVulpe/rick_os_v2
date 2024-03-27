SRC_DIR := src/asm
BUILD_DIR := build
OBJ_DIR := $(BUILD_DIR)/objects
LINKER_SCRIPT := linker.ld
ISO_DIR := $(BUILD_DIR)/iso_dir
KERNEL_PATH := $(ISO_DIR)/boot/kernel.bin
ISO_PATH := bootable.iso

SRC_FILES := $(wildcard $(SRC_DIR)/*.asm)
OBJ_FILES := $(patsubst $(SRC_DIR)/%.asm,$(OBJ_DIR)/%.o,$(SRC_FILES))

.PHONY: run $(OBJ_DIR)/libkernel.a

$(ISO_PATH): $(KERNEL_PATH) | $(ISO_DIR)
	grub-mkrescue -o $(ISO_PATH) $(ISO_DIR)


$(KERNEL_PATH): $(OBJ_FILES) $(OBJ_DIR)/libkernel.a | $(ISO_DIR)
	ld -n -o $(KERNEL_PATH) -T $(LINKER_SCRIPT) $(OBJ_FILES) $(OBJ_DIR)/libkernel.a

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)
$(OBJ_DIR):
	mkdir -p $(OBJ_DIR)

$(OBJ_DIR)/libkernel.a: 
	cargo build --release
	cp $(BUILD_DIR)/rs/x86_64-unknown-none/release/libkernel.a $(OBJ_DIR)/libkernel.a

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.asm | $(OBJ_DIR)
	nasm -o $@ -f elf64 $<


$(ISO_DIR):
	mkdir -p $(ISO_DIR)/boot/grub
	cp grub.cfg $(ISO_DIR)/boot/grub/grub.cfg



run: $(ISO_PATH)
	qemu-system-x86_64 -cdrom $(ISO_PATH) -device intel-hda -m 4G -cpu core2duo -smp 1


clean:
	rm -fr $(BUILD_DIR)
	rm -f $(ISO_PATH)
