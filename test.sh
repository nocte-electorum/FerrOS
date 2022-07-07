cargo bootimage --release --target x86_64-unknown-none
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/bootimage-ferr_os.bin.bin