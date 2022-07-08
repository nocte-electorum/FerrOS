cargo bootimage --release
qemu-system-x86_64 -drive format=raw,file=target/x64_target/release/bootimage-ferr_os.bin