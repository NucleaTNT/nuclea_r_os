build:
	cargo build
	cargo bootimage

qemu: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64-nuclea_r_os/debug/bootimage-nuclea_r_os.bin